// import { motion } from 'framer-motion'
// import Left from '../../components/Left/index.tsx'
import BT from '@/fs/blogTree';
import { useEffect } from 'react';
export default async function page(props:{bt:number}) {
  useEffect(()=>{
    let bt=BT.createFrom("F:/blog/public")!;
    console.log(bt);
  })
  return (
    <div>{'你好'}</div>

    // <motion.div>
        /* <Left/> */ 

    // </motion.div>
  )
}
