<template>
    <div>
      <t-card class="list-card-container">
        <t-row justify="space-between" class="table-bar">
          <t-col :span="3">
            <div>
              <span v-has="'{{ module_name }}:{{ table_name }}:add'">
                <t-button @click="add"> 新增 </t-button>
              </span>
              <div v-has="'{{ module_name }}:{{ table_name }}:delete'" class="delete-btn">
                <t-button variant="base" theme="danger" :disabled="!selectedRowKeys.length" @click="delBatch">删除</t-button>
                <p v-if="!!selectedRowKeys.length" class="selected-count">已选{{ selectedRowKeys.length }}项</p>
              </div>
            </div>
          </t-col>
        </t-row>
        <t-table
          row-key="id"
          :columns="columns"
          :data="dList"
          :hover="true"
          :pagination="pagination"
          :selected-row-keys="selectedRowKeys"
          :loading="isLoading"
          @page-change="(pi) => list(pi)"
          @select-change="selectChange"
        >
          <template #op="{ row }">
            <a v-has="'{{ module_name }}:{{ table_name }}:update'" class="t-button-link" @click="update(row)">修改</a>
            <span v-has="'{{ module_name }}:{{ table_name }}:delete'">
              <t-popconfirm theme="warning" content="确认删除吗" @confirm="del([row.id])">
                <a class="t-button-link">删除</a>
              </t-popconfirm>
            </span>
          </template>
        </t-table>
  
        <t-dialog v-model:visible="dialogVisible" attach="body" :header="d.id ? '修改' : '新增'" :footer="null">
          <template #body>
            <t-form ref="form" :data="d" reset-type="initial" @submit="onSubmit">{{#each attributes}}{{#unless pk}}
              <t-form-item label="{{ comment }}" name="{{ column_name }}">
                <t-input v-model="d.{{ column_name }}" placeholder="请输入{{ comment }}"></t-input>
              </t-form-item>{{/unless}}{{/each}}
              <t-form-item style="padding-top: 8px">
                <t-button theme="primary" type="submit">提交</t-button>
              </t-form-item>
            </t-form>
          </template>
        </t-dialog>
      </t-card>
    </div>
  </template>
  <script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { MessagePlugin, DialogPlugin } from 'tdesign-vue-next'
  import request from '@/utils/request'
  // 列表数据
  const columns: any = [
    { colKey: 'row-select', type: 'multiple', width: 64, fixed: 'left' },{{#each attributes }}
    { colKey: '{{ column_name }}', title: '{{ comment }}', ellipsis: true },{{/each}}
    { colKey: 'op', title: '操作', align: 'center', fixed: 'right', width: 160 },
  ]
  const form: any = ref({})
  const dList = ref([])
  const d: any = ref({})
  const dialogVisible = ref(false)
  // 分页数据
  const pagination = ref({
    current: 1,
    pageSize: 10,
    total: 0,
  })
  const isLoading = ref(false)
  const list: any = async (page = pagination.value) => {
    isLoading.value = true
    try {
      const res: any = await request.get('/api/{{ table_name }}', {
        params: { page: page.current, limit: page.pageSize },
      })
      if (res.data) {
        dList.value = res.data
        pagination.value = { ...page, total: res.total }
      }
    } catch (e) {
      console.log(e)
    } finally {
      isLoading.value = false
    }
  }
  // 初始数据
  onMounted(async () => {
    await list()
  })
  // 新增
  const add = () => {
    dialogVisible.value = true
    form.value.reset()
    form.value.clearValidate()
  }
  // 修改
  const update = (row) => {
    dialogVisible.value = true
    d.value = { ...row }
  }
  // 提交
  const onSubmit = async ({ validateResult, firstError, e }) => {
    e.preventDefault()
    if (validateResult === true) {
      const res: any = await request.post('/api/{{ table_name }}', d.value)
      if (res.code === 0) {
        MessagePlugin.success('处理成功')
        dialogVisible.value = false
        list()
      } else {
        console.log(firstError)
      }
    }
  }
  // 删除数据
  const del = async (ids) => {
    const res: any = await request.delete('/api/{{ table_name }}', { data: ids })
    if (res.code === 0) {
      MessagePlugin.success('删除成功')
      list()
    }
  }
  // 选中数据
  const selectedRowKeys = ref([])
  const selectChange = (val: number[]) => {
    selectedRowKeys.value = val
  }
  // 批量删除
  const delBatch = () => {
    const dialog = DialogPlugin({
      theme: 'warning',
      header: '确认是否删除？',
      onConfirm: () => {
        del(selectedRowKeys.value)
        dialog.hide()
      },
    })
  }
  </script>
  
  <style lang="less" scoped>
  .delete-btn {
    display: inline-block;
    margin-left: 10px;
    .selected-count {
      display: inline-block;
      margin-left: 8px;
      color: var(--td-text-color-secondary)
    }
  }
  </style>
  