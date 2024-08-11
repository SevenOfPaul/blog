import dayjs from "dayjs";
import fs from "node:fs"
import path from "node:path"
import { ismdOrDir } from "./tools";
class MD{
    modeTime:dayjs.Dayjs
    name:string
    val:Buffer
    labels:string[]
    constructor(name:string,val:Buffer,modeTime:dayjs.Dayjs,labels:string[]){
    this.name=name;
    this.modeTime=modeTime;
    this.labels=labels;
    this.val=val;
    }
}
class BT{
    modeTime:dayjs.Dayjs
    children:(BT|MD)[]
    name:string
    constructor(name:string,modeTime:dayjs.Dayjs,children?:(BT|MD)[]){
    this.name=name;
    this.children=children?children:[];
   this.modeTime=modeTime;
    }
    static createFrom(path:string){
       try{
        return  BT.addTree(path);
       }catch(e){
        console.error("出问题了",e);
       }
    }
    private static addTree(dirPath:string):BT{
        const metaInfo=fs.lstatSync(dirPath);
            let bt=new BT(dirPath,dayjs(metaInfo.mtime));
            let dirs=fs.readdirSync(dirPath).filter(fileName=>ismdOrDir(fileName));
            for(let childPath of dirs){
                //filePath 找到的子文件的path可能是dir也可能是md
               let filePath=path.join(dirPath,childPath)
              const metaInfo=fs.lstatSync(filePath);
              if(metaInfo.isDirectory()){
                bt.children.push(BT.addTree(filePath));
              }else{
                //修改 筛选出lables
                 let babels=childPath;
                 const modeTime=dayjs(metaInfo.mtime);
                 const val=fs.readFileSync(filePath);
                 bt.children.push(new MD(childPath,val,modeTime,[babels]));
              }
            }
            return bt
     }
     //@ts-ignore ts误判 已经覆盖了所有类型
   public isModify(bt1:BT|MD,bt2:BT|MD):boolean{
            if(bt1.modeTime!=bt2.modeTime){
                return false;
            }else{
                if(bt1 instanceof BT&&bt2 instanceof BT ){
                    //进入多重循环
                    for(let key in bt1.children){
                        if(bt1.children[key]&&bt2.children[key]){
                           return this.isModify(bt1.children[key],bt2.children[key]);
                        }else{
                            return false
                        }
                    }
                 }else if(bt1 instanceof MD&&bt2 instanceof MD ){
                    //判断是否都是md，是的话比对修改时间 
                    return bt1.modeTime==bt2.modeTime
                 }else{
                    return false
                 }
            }
     }
     private getMds():MD[]{
     let mds=[];
        if(this instanceof BT){
         for(let v of this.children){
            if( v instanceof BT){
               mds.push(...(v as BT).getMds());
            }else{
                mds.push(v);
            }
         }
        }
        return mds
     }
     private getLables():string[]{
        let labels=[];
           if(this instanceof BT){
            for(let v of this.children){
               if( v instanceof BT){
                labels.push(...(v as BT).getLables());
               }else{
                labels.push(...v.labels);
               }
            }
           }
           return labels
        }
        public getMdsAndLables(){
            return [this.getMds(),this.getLables()];
        }
   
}
export default BT;