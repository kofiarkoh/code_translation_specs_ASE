#include <stdio.h>
#include <math.h>

int main(int argc, char const *argv[]) {
    int N;
    //入力
    scanf("%d", &N );
    //計算
    int ans = N;
    for (size_t i = 0; i <= N; i++) {
        int count = 0;
        int t = i;
        while ( t > 0) {
            count += t %6;
            t = t/6;
        }
        t = N-i;
        while ( t > 0) {
            count += t%9;
            t = t/9;
        }
        if ( ans > count) {
            ans = count;
        }
    }
    //出力
    printf("%d\n", ans );
    return 0;
}
