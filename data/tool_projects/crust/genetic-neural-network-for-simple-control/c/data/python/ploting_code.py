import pandas as pd
import matplotlib.pyplot as plt

def plot():
    csv_file   = "TOOLBOX/PYTHON/input/data_pid_.csv"
    csv_file_2 = "TOOLBOX/PYTHON/input/data_pid_run.csv"
    image_file = "TOOLBOX/PYTHON/input/plot.png"
    
    data = pd.read_csv(csv_file)
    data2 = pd.read_csv(csv_file_2)
    
    CV = data['CV']
    RV = data['RV']
    P  = data['P']
    I  = data['I']
    D  = data['D']

    best = data2["best"]
    x  = list(range(len(CV)))

    fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(10, 8))

    ax1.plot(x, CV, label='Controlled value')
    ax1.plot(x, RV, label='Required value')

    ax1.set_title('Controll values')
    ax1.set_xlabel('time [s]')
    ax1.set_ylabel('y[t]')
    ax1.legend()
    ax1.grid(True)


    ax2.plot(x, P, label='P')
    ax2.plot(x, I, label='I')
    ax2.plot(x, D, label='D')

    ax2.set_title('PID')
    ax2.set_xlabel('time [s]')
    ax2.set_ylabel('P I D')
    ax2.legend()
    ax2.grid(True)

    ax3.plot(range(len(best)), best, label='fit')

    ax3.set_title('FIT')
    ax3.set_xlabel('generations [n]')
    ax3.set_ylabel('fit value')
    ax3.grid(True)

    

    plt.tight_layout()

    # Save the plot as an image
    plt.savefig(image_file)



if __name__ == "__main__":
  plot()