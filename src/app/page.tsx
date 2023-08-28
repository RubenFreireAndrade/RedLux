'use client'

import Image from 'next/image'
import { ChangeEvent, MouseEventHandler, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

import DateTimePicker from 'react-datetime-picker'
import 'react-datetime-picker/dist/DateTimePicker.css';
import 'react-calendar/dist/Calendar.css';
import 'react-clock/dist/Clock.css';

import axiosLib, {isCancel, AxiosError} from 'axios';

export default function Home() {

  // function handleSubmit() {
  //   axiosLib.post('http://localhost:1420/booking')
  //   .then(res => console.log(res))
  //   .catch(err => console.log(err));
  // }

  type ValuePiece = Date | null;
  type Value = ValuePiece | [ValuePiece, ValuePiece];
  
  const [value, onChange] = useState<Value>(new Date());
  
  //const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [formData, setFormData] = useState({
    fullName: '',
    location: '',
    destination: '',
    phone: '',
    importantInfo: '',
    //dateTime: '',
  });

  const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    console.log(formData);
  };

  // Used to test how to invoke multiple commands from Rust backend.
  const handleSecondSubmit = (e: React.FormEvent) => {
    console.log("2nd submit was clicked");
    e.preventDefault();
    invoke('test');
  };

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("trying_to_understand", { name }));
  // }

  return (
    <main>
      <h1>RedLux Booking Form</h1>

      {/* Might remove "preventDefault()" for easy refresh */}
      <form className="booking-info-form" onSubmit={(e) => {invoke('trying_to_understand'); handleSubmit(e); /*e.preventDefault(); /*handleSubmit(); /*greet();*/}}>
        
        <div className="customer-info-container">
          <input id="customer-name-input" name="fullName" onChange={handleChange} placeholder="Enter Customer Full Name..."/>
          <input id="location-input" name="location" onChange={handleChange} placeholder="Enter Customer Location... ( Landmark, Postcode, Establishment )"/>
          <input id="destination-input" name="destination" onChange={handleChange} placeholder="Enter Customer Destination..."/>
          <input id="phone-input" name="phone" onChange={handleChange} placeholder="Enter Customer Phone Number..."/>
          <textarea id="important-info-input" name="importantInfo" onChange={handleChange} placeholder="Enter Important Information..."/>

          {/* Hydration error for DataTimePicker */}
          {/* <DateTimePicker id="date-time-input" onChange={onChange} value={value}/> */}
          <button id="booking-submit-button" type="submit">Submit Form</button>

          <button id="booking-submit-button" onClick={(e) => {handleSecondSubmit(e)}}>Submit Form</button>
        </div>

      </form>
    </main>
  )
}
