function ismdOrDir(filename:string){
    let point=filename.indexOf(".");
    return point==-1||filename.slice(point,filename.length)==".md"
}
export {ismdOrDir}