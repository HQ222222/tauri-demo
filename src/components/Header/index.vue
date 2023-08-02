<script setup>
import { Message, Modal } from 'view-ui-plus';
import { appWindow } from '@tauri-apps/api/window'
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

let isFullScreen = ref(false)
// 最小化
const smallScreen = async () => {
    await appWindow.minimize()
}
// 最大化
// 最大化/还原
const fullScreen = async () => {
    isFullScreen.value = !isFullScreen.value
    const resizable = await appWindow.isResizable()
    if (!resizable) return
    await appWindow.toggleMaximize()
}




const exitApp = async () => {
    Modal.confirm({
        title: '退出提示',
        content: '<p>您确认退出当前应用吗？</p>',
        onOk: async () => {
            window.close();
        },
        onCancel: () => {
            Message.info('您取消了退出操作');
        }
    });
};



let helpModel = ref(false)
function userHelp() {
    helpModel.value = !helpModel.value
}

</script>
<template>
    <div class="box">
        <div class="header" data-tauri-drag-region>
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48"  viewBox="0 0 24 24">
                <defs>
                    <linearGradient id="IconifyId1813088fe1fbc01fb466" x1="-.828%" x2="57.636%" y1="7.652%" y2="78.411%">
                        <stop offset="0%" stop-color="#1989fa"></stop>
                        <stop offset="100%" stop-color="#00f2fe"></stop>
                    </linearGradient>
                </defs>
                <path fill="url(#IconifyId1813088fe1fbc01fb466)"
                    d="M3 3H21C21.5523 3 22 3.44772 22 4V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V4C2 3.44772 2.44772 3 3 3ZM7.5 11.25V9H6V15H7.5V12.75H9.5V15H11V9H9.5V11.25H7.5ZM16.25 15H17C17.5523 15 18 14.5523 18 14V10C18 9.44772 17.5523 9 17 9H14C13.4477 9 13 9.44772 13 10V14C13 14.5523 13.4477 15 14 15H14.75V16.5H16.25V15ZM14.5 10.5H16.5V13.5H14.5V10.5Z">
                </path>
            </svg>
            <!-- <div class="text-title">电脑管家</div> -->
            
            <div class="text-title">LSX</div>
            <div style="position: absolute;right: 0;top: 0;display: flex;">
                <span class="operate_item" @click="">
                    <Switch size="large" true-color="transparent" false-color="#1989fa">
                        <template #open>
                            <span>主题</span>
                        </template>
                        <template #close>
                            <span>透明</span>
                        </template>
                    </Switch>
                </span>
                <span class="operate_item" style="border-radius: 50%;" @click="userHelp">
                    <Tooltip content="帮助" placement="bottom" transfer>
                        <Icon type="md-help" size="24" />
                    </Tooltip>
                </span>
                <span class="operate_item" @click="smallScreen">
                    <Tooltip content="最小化" placement="bottom" transfer>
                        <Icon type="md-remove" size="24" />
                    </Tooltip>
                </span>
                <span class="operate_item" @click="fullScreen">
                    <Tooltip content="放大" v-show="!isFullScreen" placement="bottom" transfer>
                        <Icon type="md-qr-scanner" size="24" />
                    </Tooltip>
                    <Tooltip content="还原" v-show="isFullScreen" placement="bottom" transfer>
                        <Icon type="md-contract" size="24" />
                    </Tooltip>
                </span>

                <span class="operate_item" @click="exitApp">
                    <Tooltip content="退出应用" placement="bottom" transfer>
                        <Icon type="md-close" size="24" />
                    </Tooltip>

                </span>

            </div>
        </div>
    </div>
</template>


<style scoped>
.text-title {
    background: -webkit-linear-gradient(315deg, #1989fa 25%, #00f2fe);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    display: inline-block;
    font-size: 2.5em;
    font-weight: bold;
}

.box {
    background: transparent;
    color: #FFF;
    padding: 0.25rem;
    cursor: pointer;
    height: 3.5rem;
    background-image: linear-gradient(to right, #242424 0%, #424242 100%);
    border-radius: 5px 5px 0 0;
}

.header {
    position: relative;
    display: flex;
}

.operate_item {
    background: #AAAAAA22;
    margin: 0.5em;
    padding: 0.5em;
    border-radius: 0.5em;
    display: flex;
    justify-content: center;
    align-items: center;
}

.operate_item:hover {
    background: #AAAAAA66;
    transition: all 1s;
}

.help_text_strong {
    color: #1989fa;
    font-weight: bold;
}</style>