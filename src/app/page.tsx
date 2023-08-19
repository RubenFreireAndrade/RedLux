'use client'

import Image from 'next/image'
import { useState, useEffect } from 'react';
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

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("trying_to_understand", { name }));
  // }

  return (
    <main>
      <h1>RedLux Booking Form</h1>

      <form className="booking-info-form" onSubmit={(e) => {e.preventDefault(); invoke('trying_to_understand'); /*handleSubmit(); /*greet();*/}}>
        
        <div className="customer-info-container">
          <input id="customer-name-input" /*onChange={(e) => setName(e.currentTarget.value)}*/ placeholder="Enter Customer Full Name..."/>
          <input id="location-input" /*onChange={(e) => setName(e.currentTarget.value)}*/ placeholder="Enter Customer Location... ( Landmark, Postcode, Establishment )"/>
          <input id="destination-input" /*onChange={(e) => setName(e.currentTarget.value)}*/ placeholder="Enter Customer Destination..."/>
          <input id="phone-input" /*onChange={(e) => setName(e.currentTarget.value)}*/ placeholder="Enter Customer Phone Number..."/>
          <textarea id="important-info-input" /*onChange={(e) => setName(e.currentTarget.value)}*/ placeholder="Enter Important Information..."/>
          <DateTimePicker id="date-time-input" onChange={onChange} value={value}/>
          <button id="booking-submit-button" type="submit">Submit Form</button>
        </div>

      </form>
    </main>
  )
}
