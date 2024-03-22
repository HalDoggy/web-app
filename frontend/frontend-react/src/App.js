import logo from './logo.svg';
import React, { useState } from 'react';
import './App.css';
import Form from './Form';

function App() {
  const [products, setProducts] = useState([]);

  const handleAddProduct = (product) => {
    setProducts([...products, product]);
  };

  const handleDeleteProduct = (index) => {
    const newProducts = [...products];
    newProducts.splice(index, 1);
    setProducts(newProducts);
  };

  return (
    <div>
      {products.map((product, index) => (
        <div key={index}>
          <span>{product.name} - {product.price}円</span>
          <button onClick={() => handleDeleteProduct(index)}>削除</button>
        </div>
      ))}
      <Form
        products={products}
        onAdd={handleAddProduct}
        onDelete={handleDeleteProduct}
      />
    </div>
  );
}

export default App;
