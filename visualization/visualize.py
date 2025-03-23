import pandas as pd
import matplotlib.pyplot as plt
from adjustText import adjust_text  

file_path = 'data.xlsx'
df = pd.read_excel(file_path, sheet_name='Sheet1')


visible_columns = ['Technology', 'Matrix size', 'Read time (s)', 'Computation time (s)', 
                   'Write time (s)', 'Execution time (s)', 'Memory usage (MB)', 
                   'CPU usage (%)', 'Initialization time (s)']
df = df[visible_columns]

grouped = df.groupby('Technology')


attributes = ['Read time (s)', 'Computation time (s)', 'Write time (s)', 
              'Execution time (s)', 'Memory usage (MB)', 'CPU usage (%)', 
              'Initialization time (s)']


for attribute in attributes:
    plt.figure(figsize=(10, 6))
    texts = []  
    for i, (tech, data) in enumerate(grouped):
        
        line, = plt.plot(data['Matrix size'], data[attribute], label=tech)
        
        plt.scatter(data['Matrix size'], data[attribute], s=50, color=line.get_color())  
        
        
        for x, y in zip(data['Matrix size'], data[attribute]):
            
            if i == 0:  
                va = 'bottom'  
                y_offset = y + 0.05 
            else:  
                va = 'top'  
                y_offset = y - 0.05   
            
            text = plt.text(x, y_offset, f'{y:.4f}', fontsize=10, ha='center', va=va, color=line.get_color())
            texts.append(text)  
    
    
    adjust_text(
        texts, 
        avoid_points=True,  
        avoid_lines=True,   
    )
    
    
    x_ticks = range(500, 5501, 500)  
    plt.xticks(x_ticks)
    
    plt.xlabel('Matrix Size')
    plt.ylabel(attribute)
    plt.title(f'{attribute}')
    plt.legend()
    plt.grid(True)
    plt.show()