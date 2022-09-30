# -*- coding: utf-8 -*-
from subprocess import Popen, PIPE
from time import sleep
# 间隔时间
SLEEP_TIME = 5
# cpu空闲率低于多少时处罚
CPU_PCT = 10.0
# 写到哪个文件里
FILE_PATH = 'top_log.txt'


def main():
    with open(FILE_PATH, 'w') as f:
        pass
    while True:
        sleep(SLEEP_TIME)
        top_info = Popen(["top", "-n", "1"], stdout=PIPE)
        out, err = top_info.communicate()
        out_info = out.decode('unicode-escape')
        lines = out_info.split('\n')
        cpu_id = lines[2].split(' \x1b(B\x1b[m\x1b[39;49mid,')[0].split('ni,\x1b(B\x1b[m\x1b[39;49m\x1b[1m')[1]
        if float(cpu_id) < CPU_PCT:
            print(cpu_id)
            print('too small')
            with open(FILE_PATH, 'a') as f:
                f.writelines(out_info)
                f.write('\n------------------------------------------------------------------------------------------------------------------\n')
        else:
            print('ok')

if __name__ == "__main__":
    main()
