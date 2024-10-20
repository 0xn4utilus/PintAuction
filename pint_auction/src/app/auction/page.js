"use client"
import React, { useEffect, useState } from 'react';
import Header from '../components/header';
import Card from '../components/card';

export default function Auction() {
  const [auctionData, setAuctionData] = useState([]); // State to hold the auction data
  const [loading, setLoading] = useState(true); // State for loading status
  const [error, setError] = useState(null); // State for error handling

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch('/api/get_items');
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const data = await response.json();
        setAuctionData(data); // Set the fetched data
      } catch (error) {
        setError(error); // Set error if any occurs
      } finally {
        setLoading(false); // Set loading to false once data is fetched
      }
    };

    fetchData();
  }, []); // Empty dependency array means this effect runs once when the component mounts

  if (loading) return(
    <>
    <Header />
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center">
        <div>Loading...</div>
      </main>
    </div>
  </>

  ) // Show loading text while fetching data

  if (error) return(
    <>
    <Header />
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center">
      <div>Error: {error.message}</div>
      </main>
    </div>
  </>

  ) // Show loading text while fetching data
  

  return (
    <>
      <Header />
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        <main className="flex flex-col gap-8 row-start-2 items-center">
          {auctionData.map((item) => (
            <Card
              key={item.item_id} // Use a unique key for each Card component
              itemId={item.item_id}
              owner={item.owner}
              cost={item.cost}
              active={item.active}
            />
          ))}
        </main>
      </div>
    </>
  );
}
