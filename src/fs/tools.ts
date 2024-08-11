function ismdOrDir(filename:string){
    const point=filename.indexOf(".");
    return point==-1||filename.slice(point,filename.length)==".md"
}
export {ismdOrDir}