import React, {useState} from 'react'; 

const Form = () => {
    const [productName, setProductName] = useState('');
    const [productPrice, setProductPrice] = useState('');

    const handleAdd = () => {
        console.log(productName, productPrice);
    };

    return (
        <div>
            <input type="text" placeholder="Product Name" value={productName} onChange={(e) => setProductName(e.target.value)} />
            <input type="number" placeholder="Product Price" value={productPrice} 
            onChange={(e) => 
                {
                    if(e.target.value < 0) {
                        e.target.value = 0;
                    }
                    setProductPrice(e.target.value)
                }} 
            min="0"/>
            <button onClick={handleAdd}>Add</button>
        </div>
    );
};

export default Form;