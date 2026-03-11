
//   AtCoder Beginner Contest 072 D

#include<stdio.h>


    int p[100005];

int main(void)
{

    int i, N, a =0, t;

    scanf("%d", &N);

    for(i = 0; i < 100005; i++)
        p[i] = 0;

    for(i = 1; i <= N; i++){
        scanf("%d", &p[i]);
    }
   
    for(i = 1; i <= N-1; i++){
        if (p[i] == i) {
            t = p[i];
            p[i] = p[i+1];
            p[i+1] =t;
            a++;
            i--;
        }
    }
   
   
    printf("%d\n", a);

    return 0;
}