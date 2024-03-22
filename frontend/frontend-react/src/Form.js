import React, {useState} from 'react'; 

const Form = ({ products, onAdd, onDelete }) => {
    const [productName, setProductName] = useState('');
    const [productPrice, setProductPrice] = useState('');

    const handleAdd = () => {
        onAdd({ name: productName, price: productPrice });
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