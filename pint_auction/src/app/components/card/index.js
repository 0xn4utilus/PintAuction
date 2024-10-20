import React from 'react';

const Card = ({ itemId, cost, owner, active }) => {
  return (
    <div style={styles.card} className='w-full'>
      <div style={styles.info}>
        <h1 style={styles.itemId}>Item_id:{" "}{itemId}</h1>
        <p style={styles.cost}>Cost: {cost}</p>
        <p style={styles.owner}>Owner: {owner}</p>
      </div>
      <p style={styles.status}>Active: {active}</p>
    </div>
  );
};

const styles = {
  card: {
    backgroundColor: 'black',
    color: 'white',
    border: '2px solid white',
    borderRadius: '8px',
    padding: '20px',
    minWidth: '800px',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
    height: '200px', // Adjust height as needed
  },
  info: {
    flexGrow: 1,
  },
  itemId: {
    fontSize: '24px',
    marginBottom: '10px',
  },
  cost: {
    fontSize: '18px',
    marginBottom: '5px',
  },
  owner: {
    fontSize: '16px',
  },
  status: {
    fontSize: '16px',
    alignSelf: 'flex-end',
    marginTop: 'auto',
  },
};

export default Card;
