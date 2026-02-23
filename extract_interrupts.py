#!/usr/bin/env python3
import re
import sys

def extract_and_convert_interrupts(file_path):
    """
    从设备树文件中提取interrupts属性值，转换中断号并去重
    
    参数:
        file_path: 设备树文件路径
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"错误: 文件 {file_path} 不存在")
        return []
    except Exception as e:
        print(f"读取文件时出错: {e}")
        return []
    
    # 正则表达式匹配interrupts属性
    # 匹配类似 interrupts = <0x00 0x13 0x04>; 或 interrupts = <0x00 0xe4 0x04 0x00 0xe5 0x04 ...>;
    pattern = r'interrupts\s*=\s*<(.*?)>;'
    matches = re.findall(pattern, content, re.DOTALL)
    
    if not matches:
        print("未找到interrupts属性")
        return []
    
    interrupt_numbers = set()  # 使用集合去重
    
    for match in matches:
        # 清理空白字符和换行
        numbers_str = match.replace('\n', ' ').replace('\r', ' ')
        # 分割数字
        numbers = re.findall(r'0x[0-9a-fA-F]+|\d+', numbers_str)
        
        # 验证格式：应该是3的倍数
        if len(numbers) % 3 != 0:
            print(f"警告: 发现不规则的interrupts格式: {numbers}")
            continue
        
        # 每3个一组，取中间的数字
        for i in range(0, len(numbers), 3):
            try:
                # 取中间的数字（索引1）
                irq_str = numbers[i + 1]
                
                # 转换为整数
                if irq_str.startswith('0x'):
                    irq_num = int(irq_str, 16)
                else:
                    irq_num = int(irq_str)
                
                # 加上0x20
                irq_num += 0x20
                
                interrupt_numbers.add(irq_num)
            except (IndexError, ValueError) as e:
                print(f"警告: 处理数字时出错: {numbers[i:i+3]}, 错误: {e}")
                continue
    
    # 转换为列表并排序
    result = sorted(list(interrupt_numbers))
    return result

def main():
    if len(sys.argv) != 2:
        print("用法: python extract_interrupts.py <设备树文件路径>")
        print("示例: python extract_interrupts.py platform/aarch64/rk3568/image/dts/zone0.dts")
        sys.exit(1)
    
    file_path = sys.argv[1]
    print(f"正在处理文件: {file_path}")
    
    interrupts = extract_and_convert_interrupts(file_path)
    
    if interrupts:
        print("\n转换后的中断号列表（已去重）:")
        print("[")
        for i, irq in enumerate(interrupts):
            # 每行显示5个
            if i % 5 == 0 and i > 0:
                print()
            
            # 以十六进制格式显示
            if i == len(interrupts) - 1:
                print(f"    0x{irq:x}", end="")
            else:
                print(f"    0x{irq:x},", end="")
        
        print("\n]")
        
        print(f"\n统计信息:")
        print(f"  找到 {len(interrupts)} 个唯一的中断号")
        print(f"  最小值: 0x{min(interrupts):x}")
        print(f"  最大值: 0x{max(interrupts):x}")
        
        # 也可以以十进制显示
        print("\n十进制格式:")
        print("[")
        for i, irq in enumerate(interrupts):
            if i % 5 == 0 and i > 0:
                print()
            if i == len(interrupts) - 1:
                print(f"    {irq}", end="")
            else:
                print(f"    {irq},", end="")
        print("\n]")
    else:
        print("未找到任何中断号")

if __name__ == "__main__":
    main()