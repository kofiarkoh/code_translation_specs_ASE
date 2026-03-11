/*  ex4_3 Stone Monument
    pandacodeb  */

#include <stdio.h>

int findDepth(int west, int east);

int main(void){
    int west, east;
    scanf("%d %d", &west, &east);
    printf("%d\n", findDepth(west, east));      //雪の厚さの計算
    return 0;
}

int findDepth(int west, int east){
    int n = east - west;                        //西側と東側の残り部分の高さの差 n で、西側の高さが得られる
    int sum = 0;
    for (int i = 1; i < n; i++){
        sum += i;                               // 1+2+3+...+(n-1) の計算
    }
    int depth = sum - west;                     // depth = 西側の本当の高さ - 残り部分の高さ
    return depth;
}