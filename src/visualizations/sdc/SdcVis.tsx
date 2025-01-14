import useObjectEntryValue from "../../hooks/object_entry_value.ts";
import {Value} from "../../object_entry/types/Value.tsx";

function getY2(val: Value | undefined) {
  const closed = "31"
  const open = "16"

  return (val === undefined || val == "OPEN") ? open : closed
}

function SdcVis() {

  const pdu12 = useObjectEntryValue("power_board12", "sdc_status");
  const pdu24 = useObjectEntryValue("power_board24", "sdc_status")
  const input_board = useObjectEntryValue("input_board", "sdc_status");
  const lev1 = useObjectEntryValue("levitation_board1", "sdc_status");
  const lev2 = useObjectEntryValue("levitation_board2", "sdc_status");
  const lev3 = useObjectEntryValue("levitation_board3", "sdc_status");
  const guidanceF = useObjectEntryValue("guidance_board_front", "sdc_status");
  const guidanceB = useObjectEntryValue("guidance_board_back", "sdc_status");
  const driver_board = useObjectEntryValue("motor_driver", "sdc_status");


  return (
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 682 62">
  <defs>
  </defs>
  <g id="back">
    <line className="cls-3" y1="31" x2="75.56" y2="31"/>
    <circle className="cls-2" cx="77.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="108.5" y1="31" x2="137.56" y2="31"/>
    <circle className="cls-2" cx="139.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="170.5" y1="31" x2="199.56" y2="31"/>
    <circle className="cls-2" cx="201.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="232.5" y1="31" x2="261.56" y2="31"/>
    <circle className="cls-2" cx="263.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="294.5" y1="31" x2="323.56" y2="31"/>
    <circle className="cls-2" cx="325.22" cy="31" r="1.94"/>
    <text className="cls-1" transform="translate(81.38 50.68)"><tspan x="0" y="0" fontSize={8}>PDU24</tspan></text>
    <text className="cls-1" transform="translate(143.38 50.68)"><tspan x="0" y="0" fontSize={8}>PDU12</tspan></text>
    <text className="cls-1" transform="translate(199.56 50.68)"><tspan x="0" y="0" fontSize={8}>Motor-Driver</tspan></text>
    <text className="cls-1" transform="translate(261.56 50.68)"><tspan x="0" y="0" fontSize={8}>Input-Board</tspan></text>
    <text className="cls-1" transform="translate(317.75 50.68)"><tspan x="0" y="0" fontSize={8}>Guidance-Front</tspan></text>
    <line className="cls-3" x1="356.5" y1="31" x2="385.56" y2="31"/>
    <circle className="cls-2" cx="387.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="418.5" y1="31" x2="447.56" y2="31"/>
    <circle className="cls-2" cx="449.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="480.5" y1="31" x2="509.56" y2="31"/>
    <circle className="cls-2" cx="511.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="542.5" y1="31" x2="571.56" y2="31"/>
    <circle className="cls-2" cx="573.22" cy="31" r="1.94"/>
    <line className="cls-3" x1="604.5" y1="31" x2="682" y2="31"/>
    <text className="cls-1" transform="translate(385.56 50.68)"><tspan x="0" y="0" fontSize={8}>Levitation1</tspan></text>
    <text className="cls-1" transform="translate(447.56 50.68)"><tspan x="0" y="0" fontSize={8}>Levitation2</tspan></text>
    <text className="cls-1" transform="translate(503.75 50.68)"><tspan x="0" y="0" fontSize={8}>Guidance-Back</tspan></text>
    <text className="cls-1" transform="translate(571.56 50.68)"><tspan x="0" y="0" fontSize={8}>Levitation3</tspan></text>
  </g>
  <g id="led">
    <line className="cls-3" x1="201.22px" y1="31" x2="232.5" y2={getY2(driver_board)}/>
  </g>
  <g id="input">
    <line className="cls-3" x1="263.22" y1="31" x2="294.5" y2={getY2(input_board)}/>
  </g>
  <g id="guidance-front">
    <line className="cls-3" x1="325.22" y1="31" x2="356.5" y2={getY2(guidanceF)}/>
  </g>
  <g id="levi1">
    <line className="cls-3" x1="387.22" y1="31" x2="418.5" y2={getY2(lev1)}/>
  </g>
  <g id="levi2">
    <line className="cls-3" x1="449.22" y1="31" x2="480.5" y2={getY2(lev2)}/>
  </g>
  <g id="guidance-back">
    <line className="cls-3" x1="511.22" y1="31" x2="542.5" y2={getY2(guidanceB)}/>
  </g>
  <g id="levi3">
    <line className="cls-3" x1="573.22" y1="31" x2="604.5" y2={getY2(lev3)}/>
  </g>
  <g id="pdu24">
    <line className="cls-3" x1="77.17" y1="31" x2="108.44" y2={getY2(pdu24)}/>
  </g>
  <g id="pdu12">
    <line className="cls-3" x1="139.22" y1="31" x2="170.5" y2={getY2(pdu12)}/>
  </g>
</svg>
  );
}

//
//
//
// import { invoke } from "@tauri-apps/api";
// import { listen } from "@tauri-apps/api/event";
// import { ObjectEntryListenLatestResponse } from "../../object_entry/types/events/ObjectEntryListenLatestResponse";
// import "./SdcVis.css"
// import { ObjectEntryEvent } from "../../object_entry/types/events/ObjectEntryEvent";
// import { useEffect } from "react";
//
//
// interface OeId {
//   nodeName: string,
//   objectEntryName: string,
// }
//
// const PDU12_OE = { nodeName: "power_board12", objectEntryName: "sdc_status" };
// const PDU24_OE = { nodeName: "power_board24", objectEntryName: "sdc_status" };
// const INPUT_BOARD_OE = { nodeName: "input_board", objectEntryName: "sdc_status" };
// const MLU1_OE = { nodeName: "levitation_board1", objectEntryName: "sdc_status" };
// const MLU2_OE = { nodeName: "levitation_board2", objectEntryName: "sdc_status" };
// const MLU3_OE = { nodeName: "levitation_board3", objectEntryName: "sdc_status" };
// const MLU4_OE = { nodeName: "mlu4", objectEntryName: "sdc_status" };
// const MLU5_OE = { nodeName: "mlu5", objectEntryName: "sdc_status" };
// const MLU6_OE = { nodeName: "mlu6", objectEntryName: "sdc_status" };
// const MGU1_OE = { nodeName: "guidance_board_front", objectEntryName: "sdc_status" };
// const MGU2_OE = { nodeName: "guidance_board_back", objectEntryName: "sdc_status" };
// const MOTOR_OE = { nodeName: "motor_driver", objectEntryName: "sdc_status" };
//
// async function registerOe(oe: OeId, property: string, element: HTMLElement) {
//   const resp = await invoke<ObjectEntryListenLatestResponse>("listen_to_latest_object_entry_value", oe as any);
//   if (resp.latest !== undefined && resp.latest !== null) {
//     element.style.setProperty(property, (resp.latest.value as string == "OPEN") ? "-30deg" : "0deg");
//   }
//   const unlistenJs = await listen<ObjectEntryEvent>(resp.event_name, event => {
//     element.style.setProperty(property, (event.payload.value as string == "OPEN") ? "-30deg" : "0deg");
//   });
//
//   return () => {
//     unlistenJs();
//     invoke("unlisten_from_latest_object_entry_value", oe as any);
//   }
// }
//
// function SdcVis() {
//
//   useEffect(() => {
//
//     const svg = document.getElementById("sdc_vis")!;
//
//     let cleanup: (Promise<() => void>)[] = [];
//
//     cleanup.push(registerOe(PDU12_OE, "--pdu12", svg));
//     cleanup.push(registerOe(PDU24_OE, "--pdu24", svg));
//     cleanup.push(registerOe(INPUT_BOARD_OE, "--input_board", svg));
//     cleanup.push(registerOe(MLU1_OE, "--mlu1", svg));
//     cleanup.push(registerOe(MLU2_OE, "--mlu2", svg));
//     cleanup.push(registerOe(MLU3_OE, "--mlu3", svg));
//     cleanup.push(registerOe(MLU4_OE, "--mlu4", svg));
//     cleanup.push(registerOe(MLU5_OE, "--mlu5", svg));
//     cleanup.push(registerOe(MLU6_OE, "--mlu6", svg));
//     cleanup.push(registerOe(MGU1_OE, "--mgu1", svg));
//     cleanup.push(registerOe(MGU2_OE, "--mgu2", svg));
//     cleanup.push(registerOe(MOTOR_OE, "--motor", svg));
//
//     return () => {
//       cleanup.forEach(p => p.then(f => f()).catch(console.error));
//     };
//
//   }, []);
//
//   return (
//     <svg version="1.1" 
//       id="sdc_vis" 
//       xmlns="http://www.w3.org/2000/svg" 
//       x="0px" 
//       y="0px" 
//       viewBox="100 -10 4150 260.34"
//       >
//       <line classNameNameName="pdu24" x1="355.69" y1="187.71" x2="492.4" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="90.65" y1="187.71" x2="347.33" y2="187.71" />
//       </g>
//       <g>
//         <line classNameNameName="st0" x1="500" y1="187.71" x2="664.21" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="355.69" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="492.4" cy="187.71" r="9.09" />
//       <line classNameNameName="pdu12" x1="668.26" y1="187.71" x2="804.96" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="812.56" y1="187.71" x2="976.77" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="668.26" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="804.96" cy="187.71" r="9.09" />
//       <line classNameNameName="input_board" x1="984.38" y1="187.71" x2="1121.08" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="1128.68" y1="187.71" x2="1292.89" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="984.38" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="1121.08" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu1" x1="1296.58" y1="187.71" x2="1433.29" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="1440.89" y1="187.71" x2="1605.1" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="1296.58" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="1433.29" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu2" x1="1608.79" y1="187.71" x2="1745.49" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="1753.1" y1="187.71" x2="1917.31" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="1608.79" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="1745.5" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu3" x1="1920.99" y1="187.71" x2="2057.7" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="2065.3" y1="187.71" x2="2229.51" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="1920.99" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="2057.7" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu4" x1="2233.2" y1="187.71" x2="2369.9" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="2377.5" y1="187.71" x2="2541.71" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="2233.2" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="2369.9" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu5" x1="2545.4" y1="187.71" x2="2682.1" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="2689.7" y1="187.71" x2="2853.92" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="2545.4" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="2682.1" cy="187.71" r="9.09" />
//       <line classNameNameName="mlu6" x1="2861.52" y1="187.71" x2="2998.22" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="3005.83" y1="187.71" x2="3170.04" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="2861.52" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="2998.22" cy="187.71" r="9.09" />
//       <line classNameNameName="mgu1" x1="3173.72" y1="187.71" x2="3310.42" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="3318.02" y1="187.71" x2="3482.24" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="3173.72" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="3310.42" cy="187.71" r="9.09" />
//       <line classNameNameName="mgu2" x1="3485.91" y1="187.96" x2="3622.62" y2="187.96" />
//       <g>
//         <line classNameNameName="st0" x1="3630.22" y1="187.96" x2="3794.43" y2="187.96" />
//       </g>
//       <circle classNameNameName="st1" cx="3485.92" cy="187.96" r="9.09" />
//       <circle classNameNameName="st1" cx="3622.62" cy="187.96" r="9.09" />
//       <line classNameNameName="motor_driver" x1="3798.11" y1="187.71" x2="3934.82" y2="187.71" />
//       <g>
//         <line classNameNameName="st0" x1="3942.42" y1="187.71" x2="4256.04" y2="187.71" />
//       </g>
//       <circle classNameNameName="st1" cx="3798.11" cy="187.71" r="9.09" />
//       <circle classNameNameName="st1" cx="3934.82" cy="187.71" r="9.09" />
//       <rect x="346.6" y="43.09" classNameNameName="st2" width="188.56" height="47.75" />
//       <text transform="matrix(1 0 0 1 346.5973 85.6859)" classNameNameName="st3 st4">PDU24</text>
//       <rect x="653.65" y="43.09" classNameNameName="st2" width="188.56" height="47.75" />
//       <text transform="matrix(1 0 0 1 653.6517 85.6859)" classNameNameName="st3 st4">PDU12</text>
//       <rect x="914.81" y="43.09" classNameNameName="st2" width="291.16" height="47.75" />
//       <text transform="matrix(1 0 0 1 914.8107 85.6864)" classNameNameName="st3 st4">InputBoard</text>
//       <rect x="1295.75" y="43.09" classNameNameName="st2" width="162.6" height="47.75" />
//       <text transform="matrix(1 0 0 1 1295.7539 85.6864)" classNameNameName="st3 st4">MLU1</text>
//       <rect x="1594.49" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 1594.4901 85.6864)" classNameNameName="st3 st4">MLU2</text>
//       <rect x="1911.9" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 1911.8994 85.6864)" classNameNameName="st3 st4">MLU3</text>
//       <rect x="2225.1" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 2225.1021 85.6864)" classNameNameName="st3 st4">MLU4</text>
//       <rect x="2531.1" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 2531.0979 85.6864)" classNameNameName="st3 st4">MLU5</text>
//       <rect x="2861.52" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 2861.5176 85.6864)" classNameNameName="st3 st4">MLU6</text>
//       <rect x="3171.11" y="37.94" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 3171.1077 80.5388)" classNameNameName="st3 st4">MGU1</text>
//       <rect x="3469.82" y="43.09" classNameNameName="st2" width="165.3" height="47.75" />
//       <text transform="matrix(1 0 0 1 3469.8188 85.6864)" classNameNameName="st3 st4">MGU2</text>
//       <rect x="3720.46" y="45.32" classNameNameName="st2" width="364.89" height="47.75" />
//       <text transform="matrix(1 0 0 1 3720.4626 87.9157)" classNameNameName="st3 st4">MotorDriver</text>
//     </svg>
//   );
// }
//
export default SdcVis;
