<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted, reactive } from "vue";
import Greet from "./components/Greet.vue";
import test from "hq-lsx";
import Header from './components/Header/index.vue'
import { invoke } from "@tauri-apps/api/tauri";

let deviceInfo = reactive({
	version: '',
	risk: [],
	ip: '',
	power: {},
	net: {}
})




var userHead = reactive({name: '', color: ''})

async function initAppData() {
	// 设备型号
	let devName = await invoke("run_powershell_command", { command: "(Get-WmiObject Win32_ComputerSystemProduct).Name"})
	let devVersion = await invoke("run_powershell_command", { command: "(Get-WmiObject Win32_ComputerSystemProduct).Version"})
	deviceInfo.version = devVersion.length > devName.length ? devVersion.split('\r\n')[1] : devName.split('\r\n')[1]
	// 磁盘信息
	let riskOutput = await invoke("run_powershell_command", { command: "(Get-PSDrive -PSProvider FileSystem | Where-Object {$_.Free -ne $null})"})
	let riskLine = riskOutput.split("\n");

	riskLine.forEach((line, index) => {
		if (index > 3 && line.trim() != '') {
			let info = line.split(' ').filter(item => item.trim())
			console.log(info);
			deviceInfo.risk.push({
				name: info[0],
				used: info[1],
				free: info[2]
			})
		}
		
	})

	// ip地址
	let ipconfigOutput = await invoke("run_powershell_command", { command: "ipconfig"})
	// 初始化一个对象用于保存键值对信息
	let ipConfigInfo = {};
	// 按行解析 ipconfig 输出，并提取键值对信息
	let lines = ipconfigOutput.split("\n");

	lines.forEach((line, index) => {
		// 移除行首尾的空格
		line = line.trim();
		// console.log(line);
		// 使用冒号（:）分隔键和值
		var colonIndex = line.indexOf(':');
		if (colonIndex !== -1) {
			var key = line.substr(0, colonIndex).trim();
			var value = line.substr(colonIndex + 1).trim();
			// console.log(key);
			// 仅保留 IP 地址、子网掩码和默认网关的键值对
			if ( key.indexOf('IPv6 Address') === 0 ) {
				ipConfigInfo['IPv6 地址'] = value;
			}

			if (key.indexOf('IPv4 Address') === 0 ) {
				ipConfigInfo['IPv4 地址'] = value;
			}
			if (key.indexOf('Subnet Mask') === 0 ) {
				ipConfigInfo['子网掩码'] = value;
			}
			if ( key.indexOf('Default Gateway') === 0 && value) {
				ipConfigInfo['网关1'] = value
				ipConfigInfo['网关2'] = lines[index + 1].trim()
			}
		}
		
	});
	deviceInfo.ip = ipConfigInfo

	// 电池信息
	let powerOutput = await invoke("run_powershell_command", { command: "Get-WmiObject -Class Win32_Battery"})
	let powerLine = powerOutput.split("\n");
	let powerInfo = {}
	powerLine.forEach(line => {
		var colonIndex = line.indexOf(':');
		if (colonIndex !== -1) {
			var key = line.substr(0, colonIndex).trim();
			var value = line.substr(colonIndex + 1).trim();
			if (key === 'EstimatedChargeRemaining') {
				powerInfo['电量'] = value
			}
			if (key === 'EstimatedRunTime') {
				powerInfo['可用时间'] = (value > 10000 ? '充电中...' : (value + '分钟'))
			}
		}
	})
	deviceInfo.power = powerInfo

	// 网络信息
	let netOutput = await invoke("run_powershell_command", { command: "Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Sort-Object -Property InterfaceIndex | Select-Object -First 1"})
	let netLine = netOutput.split("\n");
	let netInfo = {}
	netLine.forEach((line, index) => {
		if (index > 3 && line.trim() != '') {
			let info = line.split(' ').filter(item => item.trim())
			console.log(info);
			deviceInfo.net = {
				'上网网卡': info[0],
				'物理地址': info [info.length-3],
				'网络速度': 0,
				'上行带宽': info[info.length-2] + ' ' + info[info.length-1]
				
			}
		}
	})
	getNetWork()

	showWinAnomation()




	
	

}

initAppData()


function getNetWork() {
	if (navigator.connection) {
		var connection = navigator.connection;

		// 判断是否有网络连接
		if (connection.type === 'none') {
			console.log('没有网络连接');
		} else {
			// 获取设备的网络类型
			console.log('网络类型:', connection);

			deviceInfo.net['网络类型'] = connection.effectiveType || '未知'
			// 获取设备的往返时延估计（以毫秒为单位）
			if (connection.rtt) {
				console.log('往返时延估计:', connection.rtt);
				// deviceInfo.net['网络时延'] = connection.rtt + ' 毫秒'
			}

			// 获取设备的下行链接的实际带宽估计（以Mbps为单位）
			if (connection.downlink) {
				// 将Mbps转换为KB/s
				var speedInKbps = connection.downlink * 1024;
				deviceInfo.net['网络速度'] = speedInKbps.toFixed(2) + ' KB/s'
				// console.log('下载带宽估计:', speedInKbps.toFixed(2) + ' KB/s');
			}
		}
	} else {
		console.log('不支持获取网络信息的API');
	}

}

function nameBg(userName) {
	

	test()
	// 获取名字第一个文字，转换成16进制颜色值

	let firstName = userName.substring(1, 0);
	// 颜色
	function tranColor(userName)  {
		var str = '';
		for(var i=0; i<userName.length; i++) {
			str += parseInt(userName[i].charCodeAt(0), 10).toString(16);
		}
		return '#' + str.slice(1, 4);
	}
	
	const bgColor = tranColor(userName)
	console.log('变化了');
	// userHead =  {name: firstName, color: bgColor}
	userHead.name = firstName
	userHead.color = bgColor
	// return {name: firstName, color: bgColor}
}
nameBg("黄庆")
console.log(userHead);

import lottie from 'lottie-web';



function showWinAnomation() {
	// const container = document.getElementById('lottie-container');
    // if (container) {
    //   const animation = lottie.loadAnimation({
    //     container: container,
    //     renderer: 'svg',
    //     loop: false,
    //     autoplay: true,
	// 	// 动画json文件
    //     path: '/win.json'
    //   });
    // }

	const container2 = document.getElementById('lottie-container2');
    if (container2) {
      const animation = lottie.loadAnimation({
        container: container2,
        renderer: 'svg',
        loop: true,
        autoplay: true,
		// 动画json文件
        path: '/bear.json'
      });
    }

	const container3 = document.getElementById('lottie-container3');
    if (container3) {
      const animation = lottie.loadAnimation({
        container: container3,
        renderer: 'svg',
        loop: true,
        autoplay: true,
		// 动画json文件
        path: '/city.json'
      });
    }

	// const container4 = document.getElementById('lottie-container4');
    // if (container4) {
    //   const animation = lottie.loadAnimation({
    //     container: container4,
    //     renderer: 'svg',
    //     loop: true,
    //     autoplay: true,
	// 	// 动画json文件
    //     path: '/play.json'
    //   });
    // }
}


onMounted(()=> {
	

	
})


</script>

<template>
	<div class="app_container" >
		<Header></Header>
		<div style="text-align: center;position: absolute;">
			<div id="lottie-container3" style="display: inline-block;width: 110vw;height: 100vh;"></div>
		</div>
		
		<div class="container">
			<!-- <div style="">
				<h1 class="text-title">欢迎进入HQ电脑管家!</h1>
				<div>{{ deviceInfo.version }}</div>
				<div v-for="(item, key) in deviceInfo.ip">
					<div>{{ key }}:{{item}}</div>	
				</div>
				<div v-for="item in deviceInfo.risk"> 
					<div>{{ item.name }}:{{ item.used }}GB / {{ item.free }}GB</div>	
				</div>
				<div v-for="(item, key) in deviceInfo.power">
					<div>{{ key }}:{{item}}</div>	
				</div>
				<div v-for="(item, key) in deviceInfo.net">
					<div>{{ key }}:{{item}}</div>	
				</div>
				
				<div style="text-align: center;">
					<div id="lottie-container" style="width: 400px;height:200px;display: inline-block;"></div>
				</div>
				<div id="title" style="margin: 10px 0;">
					<div class="row">
						<div class="head" :style="{'background': userHead.color}">{{ userHead.name }}</div>
					</div>
					<div style="margin-top: 1em;">
						七七·码生
					</div>
				</div>

				
			</div> -->
			<Grid :border="false" :hover="true" :col="3">
				<GridItem style="width: 100%;" >
					<div style="text-align: left;">
						<Row>
							<Col span="12">
								<h4 class="text-title" style="font-size: 2em;">
									
									<Icon size="40" style="" type="md-aperture" />{{ deviceInfo.version }}
								</h4>
							</Col>
							<Col span="12" style="display: flex;justify-content:right;align-items:center;">
								<span>
									<Icon size="40" color="#19be6b" type="md-battery-charging" />
									<!-- <Icon  type="ios-battery-charging" /> -->
								</span>
								<div style="display: inline-block;font-size: 0.5em;">
									<div>
										电量：<strong>{{deviceInfo.power.电量}}</strong> %
									</div>
									<div>
										{{ deviceInfo.power.可用时间 }}
									</div>
								</div>
							</Col>
						</Row>
					</div>
				</GridItem>
				<GridItem style="">
					<div style="text-align: left;position: relative;">
						<span class="text_bg">磁盘</span>
						<div v-for="item in deviceInfo.risk"> 
							<div>
								<span style="display: inline-block;">
									<div style="letter-spacing: 0.5em;">
										<Icon type="ios-folder" size="24"/>
										<span style="font-size: 1.5em;font-weight: bold;">{{ item.name }}盘</span>
									</div>
									<div>
										<strong style="color: #1989fa;">{{ item.used }}</strong> GB / 
										<strong style="color: #19be6b;">{{ item.free }}</strong> GB
									</div>
								</span>
								
							</div>	
						</div>
					</div>
					
				</GridItem>
				<GridItem >
					<div style="text-align: left;position: relative;">
						<span class="text_bg">IP</span>
						<Icon type="logo-rss" />
						<div v-for="(item, key) in deviceInfo.ip">
							<div>
								<span style="font-size: 0.8em;">{{ key }}: </span>
								<span style="font-weight: bold;">{{item}}</span>
							</div>	
						</div>
					</div>
					
				</GridItem>
				<GridItem >
					<div style="height: 100%;text-align: left;position: relative;">
						<span class="text_bg">网卡</span>
						<!-- <Icon type="ios-globe-outline" /> -->
						<div v-for="(item, key) in deviceInfo.net">
							<div>
								<span style="font-size: 0.8em;">{{ key }}: </span>
								<span style="font-weight: bold;">{{item}}</span>
							</div>
						</div>
					</div>
					
				</GridItem>
				<GridItem >
					<div class="">
						<div style="text-align: center;">
							<!-- <button>显示动画</button> -->
							<div id="lottie-container" style="display: inline-block;"></div>
						</div>
					</div>
					
				</GridItem>
				<GridItem >
					<div style="text-align: center;position: relative;">
						<div id="lottie-container2" style="display: inline-block;"></div>
						<div style="position: absolute;bottom:1em;left:50%;transform: translateX(-50%);">
							小熊熊为你努力加载
						</div>
					</div>
				</GridItem>
				<GridItem >
					<div style="text-align: center;position: relative;">
						<div id="lottie-container4" style="display: inline-block;"></div>
					</div>
				</GridItem>
			</Grid>
		
		</div>
	</div>
	
</template>

<style scoped>
.app_container {
	/* background-color: #1989fa; */
	height: 100vh;
	background-image: linear-gradient(to top, #dfe9f3 0%, white 100%);
	border-radius: 10px;
	position: relative;
	/* gap: 20px;
    
    backdrop-filter: blur(30px);
    border: 2px ;*/
}
.head {
	width: 2em;
	height: 2em;
	font-weight: bold;
	font-size: 2em;
	line-height: 2em;
	border-radius: 1em;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.text-title {
	background: -webkit-linear-gradient(315deg,#42d392 25%,#647eff);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
	letter-spacing: 0.1em;
}

.block_bg_glass {
    gap: 20px;
    border-radius: 10px;
    backdrop-filter: blur(3px);
	background-color: #ffffff22;
    border: 2px rgba(255,255,255,0.4) solid;
    border-bottom: 2px rgba(40,40,40,0.35) solid;
    border-right: 2px rgba(40,40,40,0.35) solid;
    position: relative;
    box-shadow: rgba(0, 0, 0, 0.3) 1px 4px 4px;
}

.text_bg {
	position: absolute;
	bottom: 0;
	right: 0; 
	font-size: 4rem;
	font-weight: bold;
	opacity: 0.2;
}


</style>
