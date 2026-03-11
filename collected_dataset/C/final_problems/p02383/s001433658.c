#include <stdio.h>
#include <string.h>

// 展開図から得られるサイコロシミュレーションを行うプログラム

int main() {
    int iDice[7];   // ダイスの整数
    int iCount;     // counter
    int iSwapTmp;   // 転がした際に目を変更するためのテンポラリ
    char szDiceRole[110];   // 転がす命令

    // In: ダイスに書かれた整数
    for (iCount = 1; iCount <= 6; iCount++) {
        scanf("%d", &iDice[iCount]);
    }
    // In: 転がし方の命令
    scanf("%s", szDiceRole);
    // 命令数分繰り返す
    iCount = 0;
    while (szDiceRole[iCount] != '\0') {
        switch (szDiceRole[iCount]) {
        case 'E':
            iSwapTmp = iDice[1];    // 上面を退避
            iDice[1] = iDice[4];    // 左面->上面
            iDice[4] = iDice[6];    // 下面->左面
            iDice[6] = iDice[3];    // 右面->下面
            iDice[3] = iSwapTmp;    // 上面->右面
            break;
        case 'W':
            iSwapTmp = iDice[1];    // 上面を退避
            iDice[1] = iDice[3];    // 右面->上面
            iDice[3] = iDice[6];    // 下面->右面
            iDice[6] = iDice[4];    // 左面->下面
            iDice[4] = iSwapTmp;    // 上面->左面
            break;
        case 'S':
            iSwapTmp = iDice[1];    // 上面を退避
            iDice[1] = iDice[5];    // 奥->上
            iDice[5] = iDice[6];    // 下->奥
            iDice[6] = iDice[2];    // 手前->下
            iDice[2] = iSwapTmp;    // 上->手前
            break;
        case 'N':
            iSwapTmp = iDice[1];    // 上面を退避
            iDice[1] = iDice[2];    // 手前->上
            iDice[2] = iDice[6];    // 下->手前
            iDice[6] = iDice[5];    // 奥->下
            iDice[5] = iSwapTmp;    // 上->奥
            break;
        default:
            break;
        }
        iCount++;
    }
    printf("%d\n", iDice[1]);

    return 0;
}
