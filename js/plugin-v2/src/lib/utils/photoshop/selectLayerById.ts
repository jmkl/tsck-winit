import { action } from "photoshop";

export async function selectLayerById(id:number){
	await action.batchPlay([{
   "_obj": "select",
   "_target": [
      {
         "_ref": "layer",
         "_id": id
      }
   ],

}],{})

}
