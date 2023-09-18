'use client'

import Image from 'next/image'
import { ChangeEvent, MouseEventHandler, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

import axiosLib, { isCancel, AxiosError } from 'axios';
import { error } from 'console';

export default function Home() {
  const [formData, setFormData] = useState({
    full_name: '',
    location_of_collection: '',
    location_of_destination: '',
    date_time_of_collection: '',
    date_time_of_destination: '',
    phone_number: '',
    important_information: '',
  });

  const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: e.target.value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log(formData);

    await invoke('listen_submit', { formData })
      .then((res) => {
        console.log('Success: ', res);
      })
      .catch((err) => {
        console.log('Error: ', err);
      });
  };

  return (
    <main>
      <h1>RedLux Booking Form</h1>

      <form className="booking-info-form" onSubmit={(e) => { handleSubmit(e); }}>

        <div className="customer-info-container">
          <input id="customer-name-input" name="full_name" onChange={handleChange} placeholder="Enter Customer Full Name..." />
          <input id="location-input" name="location_of_collection" onChange={handleChange} placeholder="Enter Customer Location... ( Landmark, Postcode, Establishment )" />
          <input id="destination-input" name="location_of_destination" onChange={handleChange} placeholder="Enter Customer Destination..." />
          <label>Date & Time for Collection:</label>
          <input id="date-time-input" type="datetime-local" name="date_time_of_collection" onChange={handleChange} />
          <label>Date & Time for Destination:</label>
          <input id="date-time-input" type="datetime-local" name="date_time_of_destination" onChange={handleChange} />
          <input id="phone-input" name="phone_number" onChange={handleChange} placeholder="Enter Customer Phone Number..." />
          <textarea id="important-info-input" name="important_information" onChange={handleChange} placeholder="Enter Important Information..." />
          <button id="booking-submit-button" type="submit">Submit Form</button>
        </div>

      </form>
    </main>
  )
}
