#include <stdio.h>

int main(void) {
    int F; // 華氏温度
    int C; // 摂氏温度
    
    scanf("%d", &F);
    C = (F - 30) / 2;
    
    printf("%d\n", C);
    
    return 0;
}
