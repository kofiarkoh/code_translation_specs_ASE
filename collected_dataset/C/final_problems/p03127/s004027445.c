#include<stdio.h>

int main(void){
    int N;
    scanf("%d", &N);
    int A[N];
    int i;
    int min = 1000000001;
    int minIdx = 0;
    for(i = 0; i < N; i++){
        scanf("%d", &A[i]);
        if(min > A[i]){min = A[i]; minIdx = i;}
    }
    //printf("最初のmin:%d\n", min);
    int sum = 0;
    int flag = 0;//少なくとも1があるかどうか/1:yes
    int ans;
    int cnt = 0;
    for(;;){
        for(i = 0; i < N; i++){
            if(i == minIdx)
                continue;
            A[i] %= min;
            sum += A[i];
            if(A[i] == 1){flag = 1;}
            //if(A[i] == 0){cnt++;}
            //ここでminを入れ替えると同じ段階での割る数が異なってしまう
            //→N回のループを終えてからminを求めるべきかなと
            //if((min > A[i])&&(A[i]!=0)){min = A[i];}
        }
        //for(i = 0; i < N; i++){printf("%d ", A[i]);}printf("\n");
        for(i = 0; i < N; i++){if((min > A[i])&&(A[i]!=0)){min = A[i]; minIdx = i;}}
        //printf("次のmin:%d\n", min);
        if(sum == 0){ans = min; break;}
        if(flag == 1){ans = 1; break;}
        sum = 0;
    }
    printf("%d\n", ans);
    return 0;
}