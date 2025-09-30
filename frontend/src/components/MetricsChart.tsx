import React from 'react'
import { 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  Legend, 
  ResponsiveContainer 
} from 'recharts'
import { useTheme } from '@mui/material/styles'

// 模拟数据
const generateData = () => {
  const data = []
  const now = new Date()
  
  for (let i = 23; i >= 0; i--) {
    const time = new Date(now.getTime() - i * 60 * 60 * 1000)
    data.push({
      time: time.getHours() + ':00',
      blockTime: 6.0 + Math.random() * 0.8 - 0.4,
      transactions: 40 + Math.random() * 20,
      nodes: 150 + Math.random() * 20,
      healthScore: 85 + Math.random() * 10
    })
  }
  
  return data
}

const MetricsChart: React.FC = () => {
  const theme = useTheme()
  const data = generateData()

  return (
    <ResponsiveContainer width="100%" height={400}>
      <LineChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
        <CartesianGrid 
          strokeDasharray="3 3" 
          stroke={theme.palette.divider}
          opacity={0.3}
        />
        <XAxis 
          dataKey="time" 
          stroke={theme.palette.text.secondary}
          fontSize={12}
        />
        <YAxis 
          stroke={theme.palette.text.secondary}
          fontSize={12}
        />
        <Tooltip 
          contentStyle={{
            backgroundColor: theme.palette.background.paper,
            border: `1px solid ${theme.palette.divider}`,
            borderRadius: theme.shape.borderRadius,
            color: theme.palette.text.primary
          }}
          labelStyle={{ color: theme.palette.text.primary }}
        />
        <Legend />
        <Line 
          type="monotone" 
          dataKey="blockTime" 
          stroke={theme.palette.primary.main}
          strokeWidth={2}
          name="出块时间 (s)"
          dot={{ fill: theme.palette.primary.main, strokeWidth: 2, r: 3 }}
          activeDot={{ r: 5, stroke: theme.palette.primary.main, strokeWidth: 2 }}
        />
        <Line 
          type="monotone" 
          dataKey="transactions" 
          stroke={theme.palette.secondary.main}
          strokeWidth={2}
          name="交易数/分钟"
          dot={{ fill: theme.palette.secondary.main, strokeWidth: 2, r: 3 }}
          activeDot={{ r: 5, stroke: theme.palette.secondary.main, strokeWidth: 2 }}
        />
        <Line 
          type="monotone" 
          dataKey="healthScore" 
          stroke="#4caf50"
          strokeWidth={2}
          name="健康度"
          dot={{ fill: '#4caf50', strokeWidth: 2, r: 3 }}
          activeDot={{ r: 5, stroke: '#4caf50', strokeWidth: 2 }}
        />
      </LineChart>
    </ResponsiveContainer>
  )
}

export default MetricsChart