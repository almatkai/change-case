export let preferences={};export const calls=[];export let clipboard='',selected='',selectionError=false,clipboardError=false;
export function setup(v={}){preferences=v.preferences??{};clipboard=v.clipboard??'';selected=v.selected??'';selectionError=v.selectionError??false;clipboardError=v.clipboardError??false;calls.length=0;}
export const getPreferenceValues=()=>preferences;
export const Clipboard={readText:async()=>{calls.push(['clipboard:read']);if(clipboardError)throw Error('clipboard');return clipboard;},copy:async v=>{calls.push(['clipboard:write',v]);},paste:async v=>{calls.push(['core:platform:paste-text',v]);}};
export async function getSelectedText(){calls.push(['core:platform:selected-text']);if(selectionError)throw Error('selection');return selected;}
export const popToRoot=async()=>{calls.push(['popToRoot']);};export const showHUD=async v=>{calls.push(['HUD',v]);};
