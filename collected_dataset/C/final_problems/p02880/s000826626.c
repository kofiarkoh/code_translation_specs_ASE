// 課題3_4 AtCoder Beginner Contest 144 B - 81
// nk777
 
#include <stdio.h>
 
int main(void){
    // 整数Nを格納n,カウンタi, 結果格納result
    int n, i, result = 0;
 
    // nを受け取る
    scanf("%d", &n);

    for (i = 1; i < 10; i++)
    {
        // iの段を調べる
        // n/iの商が整数で1以上9以下
        if (n % i == 0 && 0 < n / i && n / i < 10)
        {
            result = 1;
            break;
        }
    }
    if (result == 1)
    {
        printf("Yes\n");
    }
    else
    {
        printf("No\n");
    }
    
    return 0;
}