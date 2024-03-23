import React, {useState} from 'react'; 

const Form = ({ products, onAdd, onDelete }) => {
    const [productName, setProductName] = useState('');
    const [productPrice, setProductPrice] = useState('');

    const postData = async () => {
        const url = 'http://localhost:8765/add';
        const data = { name: productName, price: parseInt(productPrice) }; // 商品情報を設定
      
        try {
          const response = await fetch(url, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(data)
          });
      
          if (response.ok) {
            const result = await response.text();
            console.log('サーバーからのレスポンス:', result);
          } else {
            console.error('エラーが発生しました:', response.status);
          }
        } catch (error) {
          console.error('ネットワークエラー:', error);
        }
        
        onAdd();
      };

    const handleAdd = () => {
        postData();
        setProductName('');
        setProductPrice('');
    };

    return (
        <div>
            <input type="text" placeholder="Product Name" 
            value={productName} 
            onChange={(e) => setProductName(e.target.value)}/>

            <input type="number" placeholder="Product Price" value={productPrice} 
            onChange={(e) => 
                {
                    if(e.target.value < 0) {
                        e.target.value = 0;
                    }
                    setProductPrice(e.target.value)
                }} 
            min="0"/>
            <button onClick={handleAdd}>追加</button>
        </div>
    );
};

export default Form;