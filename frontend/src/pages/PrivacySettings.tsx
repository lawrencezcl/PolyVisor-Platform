import React, { useState } from 'react'
import React, { useState } from 'react'
import {
  Box,
  Typography,
  Card,
  CardContent,
  FormControl,
  FormLabel,
  FormGroup,
  FormControlLabel,
  Switch,
  Select,
  MenuItem,
  Button,
  Alert,
  Chip,
  Grid,
  Slider
} from '@mui/material'

const PrivacySettings: React.FC = () => {
  const [settings, setSettings] = useState({
    allowAnalytics: true,
    allowSharing: false,
    dataRetention: 30,
    privacyLevel: 'protected',
    anonymization: true
  })

  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle')

  const handleSettingChange = (key: string, value: any) => {
    setSettings(prev => ({ ...prev, [key]: value }))
  }

  const handleSave = async () => {
    setSaveStatus('saving')
    try {
      // 模拟保存
      await new Promise(resolve => setTimeout(resolve, 1000))
      setSaveStatus('saved')
      setTimeout(() => setSaveStatus('idle'), 3000)
    } catch (error) {
      setSaveStatus('error')
    }
  }

  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        隐私设置
      </Typography>
      <Typography variant="body1" color="text.secondary" paragraph>
        管理您的数据隐私偏好和保护级别
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                数据处理偏好
              </Typography>
              
              <FormGroup>
                <FormControlLabel
                  control={
                    <Switch
                      checked={settings.allowAnalytics}
                      onChange={(e) => handleSettingChange('allowAnalytics', e.target.checked)}
                    />
                  }
                  label="允许数据分析"
                />
                <Typography variant="body2" color="text.secondary" sx={{ mb: 2, ml: 4 }}>
                  允许使用您的数据进行网络性能分析和改进
                </Typography>

                <FormControlLabel
                  control={
                    <Switch
                      checked={settings.allowSharing}
                      onChange={(e) => handleSettingChange('allowSharing', e.target.checked)}
                    />
                  }
                  label="允许数据共享"
                />
                <Typography variant="body2" color="text.secondary" sx={{ mb: 3, ml: 4 }}>
                  允许与研究机构和合作伙伴共享匿名化数据
                </Typography>

                <FormControlLabel
                  control={
                    <Switch
                      checked={settings.anonymization}
                      onChange={(e) => handleSettingChange('anonymization', e.target.checked)}
                    />
                  }
                  label="启用数据匿名化"
                />
                <Typography variant="body2" color="text.secondary" sx={{ mb: 3, ml: 4 }}>
                  使用零知识证明和差分隐私技术保护数据
                </Typography>
              </FormGroup>

              <FormControl fullWidth sx={{ mb: 3 }}>
                <FormLabel>隐私级别</FormLabel>
                <Select
                  value={settings.privacyLevel}
                  onChange={(e) => handleSettingChange('privacyLevel', e.target.value)}
                  size="small"
                >
                  <MenuItem value="public">公开</MenuItem>
                  <MenuItem value="protected">受保护</MenuItem>
                  <MenuItem value="private">私有</MenuItem>
                  <MenuItem value="sensitive">敏感</MenuItem>
                </Select>
              </FormControl>

              <Box sx={{ mb: 3 }}>
                <FormLabel>数据保留期限（天）</FormLabel>
                <Slider
                  value={settings.dataRetention}
                  onChange={(_, value) => handleSettingChange('dataRetention', value)}
                  min={7}
                  max={365}
                  step={7}
                  marks={[
                    { value: 7, label: '7天' },
                    { value: 30, label: '30天' },
                    { value: 90, label: '90天' },
                    { value: 365, label: '1年' }
                  ]}
                  valueLabelDisplay="auto"
                  sx={{ mt: 2 }}
                />
              </Box>

              <Box display="flex" gap={2}>
                <Button
                  variant="contained"
                  onClick={handleSave}
                  disabled={saveStatus === 'saving'}
                >
                  {saveStatus === 'saving' ? '保存中...' : '保存设置'}
                </Button>
                <Button variant="outlined">
                  重置为默认值
                </Button>
              </Box>

              {saveStatus === 'saved' && (
                <Alert severity="success" sx={{ mt: 2 }}>
                  隐私设置已保存
                </Alert>
              )}
              {saveStatus === 'error' && (
                <Alert severity="error" sx={{ mt: 2 }}>
                  保存失败，请重试
                </Alert>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                当前隐私状态
              </Typography>
              
              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                  隐私级别
                </Typography>
                <Chip 
                  label={
                    settings.privacyLevel === 'public' ? '公开' :
                    settings.privacyLevel === 'protected' ? '受保护' :
                    settings.privacyLevel === 'private' ? '私有' : '敏感'
                  }
                  color={
                    settings.privacyLevel === 'public' ? 'default' :
                    settings.privacyLevel === 'protected' ? 'primary' :
                    settings.privacyLevel === 'private' ? 'secondary' : 'error'
                  }
                />
              </Box>

              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                  保护措施
                </Typography>
                <Box display="flex" flexWrap="wrap" gap={1}>
                  {settings.anonymization && (
                    <Chip label="零知识证明" size="small" color="success" />
                  )}
                  <Chip label="差分隐私" size="small" color="success" />
                  <Chip label="K-匿名性" size="small" color="success" />
                  <Chip label="数据泛化" size="small" color="success" />
                </Box>
              </Box>

              <Box>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                  数据权限
                </Typography>
                <Box display="flex" flexDirection="column" gap={1}>
                  <Chip 
                    label={`分析: ${settings.allowAnalytics ? '允许' : '禁止'}`}
                    color={settings.allowAnalytics ? 'success' : 'default'}
                    size="small"
                  />
                  <Chip 
                    label={`共享: ${settings.allowSharing ? '允许' : '禁止'}`}
                    color={settings.allowSharing ? 'success' : 'default'}
                    size="small"
                  />
                  <Chip 
                    label={`保留期: ${settings.dataRetention}天`}
                    color="info"
                    size="small"
                  />
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  )
}

export default PrivacySettings