#include <stdio.h>

// ルンルン数をスタックにpush
void lunlunPush(unsigned long long num, unsigned long long* stack)
{
    static int i = 0;
    // 桁が変わるごとにスタックをリセット
    if (num == 0) {
        i = 0;
        return;
    }
    stack[i] = num;
    i++;
    return;
}


// N桁の数のルンルン数をスタックに入れながら数え上げる
int lunlunCount(int N, int start, int end, unsigned long long num, unsigned long long *stack)
{
    if (N == 0) {
        lunlunPush(num, stack);
        // printf("%d ", num);
        return 1;
    }
    int S = 0;
    int E = 0;
    int total = 0;
    int i;
    for (i = start; i <= end; i++) {
        S = (i - 1) < 0 ? 0 : (i - 1);
        E = (i + 1) > 9 ? 9 : (i + 1);
        total += lunlunCount((N - 1), S, E, (num * 10) + i, stack);
    }
    return total;
}

int main(void)
{
    int K;
    int N, X;
    // 10^5番目のルンルン数は10桁、10桁のルンルン数は130900個
    unsigned long long lunlunStack[130900];

    // input
    scanf("%d", &K);

    // K番目のルンルン数 -> N桁の数のK'番目のルンルン数
    N = 1;
    X = 0;
    while (1) {
        X = lunlunCount(N, 1, 9, 0, lunlunStack);
        if (X >= K) {
            break;
        } else {
            // スタックの添字をリセット
            lunlunPush(0, lunlunStack);
            // printf("\n");
            K -= X;
            N++;
        }
    }
    // printf("%d桁のルンルン数は%d個\n", N, X);
    // printf("答えは%d桁のルンルン数の%d番目の数\n", N, K);

    // output
    // 添字は0から始まるのでスタックのK-1番目
    printf("%lld\n", lunlunStack[K-1]);

    return 0;
}