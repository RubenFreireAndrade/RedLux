'use client'

import Image from 'next/image'
import { ChangeEvent, MouseEventHandler, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

import DateTimePicker from 'react-datetime-picker'
import 'react-datetime-picker/dist/DateTimePicker.css';
import 'react-calendar/dist/Calendar.css';
import 'react-clock/dist/Clock.css';

import axiosLib, {isCancel, AxiosError} from 'axios';
import { error } from 'console';

export default function Home() {

  type ValuePiece = Date | null;
  type Value = ValuePiece | [ValuePiece, ValuePiece];
  
  const [value, onChange] = useState<Value>(new Date());

  const [name, setName] = useState("");

  const [formData, setFormData] = useState({
    full_name: '',
    location_of_collection: '',
    location_of_destination: '',
    // date_time_of_collection: '',
    // date_time_of_destination: '',
    phone_number: '',
    important_information: '',
  });

  const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log(formData);

    await invoke('listen_submit', { formData });
  };

  // Used to test how to invoke multiple commands from Rust backend.
  const handleSecondSubmit = (e: React.FormEvent) => {
    console.log("2nd submit was clicked");
    e.preventDefault();
    invoke('test');
  };

  return (
    <main>
      <h1>RedLux Booking Form</h1>

      {/* Might remove "preventDefault()" for easy refresh */}
      <form className="booking-info-form" onSubmit={(e) => {handleSubmit(e); /*handleSubmit();*/}}>
        
        <div className="customer-info-container">
          <input id="customer-name-input" name="full_name" onChange={handleChange} placeholder="Enter Customer Full Name..."/>
          <input id="location-input" name="location_of_collection" onChange={handleChange} placeholder="Enter Customer Location... ( Landmark, Postcode, Establishment )"/>
          <input id="destination-input" name="location_of_destination" onChange={handleChange} placeholder="Enter Customer Destination..."/>
          <input id="phone-input" name="phone_number" onChange={handleChange} placeholder="Enter Customer Phone Number..."/>
          <textarea id="important-info-input" name="important_information" onChange={handleChange} placeholder="Enter Important Information..."/>

          {/* Hydration error for DataTimePicker */}
          {/* <DateTimePicker id="date-time-input" onChange={onChange} value={value}/> */}
          <button id="booking-submit-button" type="submit">Submit Form</button>

          <button id="booking-submit-button" onClick={(e) => {/*handleSecondSubmit(e)*/}}>Submit Form</button>
        </div>

      </form>
    </main>
  )
}
