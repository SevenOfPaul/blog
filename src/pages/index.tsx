import BT from "@/fs/blogTree";
export async function getServerSideProps() {
   let bt=BT.createFrom("F:/blog/public");
  return {
    props:{
      bt:JSON.stringify(bt)
    }
  };
}
export default function Home(props:{bt:JSON}) {

  return (
    <main>
      <p>hello</p>
      <p>{props.bt}</p>
    </main>
  );
}

