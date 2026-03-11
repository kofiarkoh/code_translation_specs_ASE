#include <stdio.h>

int main()
{
    int a,b,i,N,flag = 0;
    scanf("%d %d", &a, &b);
    scanf("%d", &N);
    int s[N];
    int f[N];

    for(i=0;i<N;++i)
      scanf("%d %d", &s[i], &f[i]);

    for(i=0;i<N;++i) {
        if((a<=s[i] && b>s[i]) || (a<f[i] && b>=f[i])){
            flag = 1;
            break;
        }else if(a>=s[i] && b<=f[i]) {
            flag = 1;
            break;
        }
    }
    printf("%d\n", flag);

    return 0;
}

