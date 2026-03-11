#include <stdio.h>
#define NUM 100001

int main(void) {
    int i,n,ans,ans2,ans3,num1,num2;
    int v[NUM] = {0};
    int numodd[NUM] = {0};//奇数用の配列 
    int numeven[NUM] = {0};//偶数用の配列
    int oddmax,evenmax,oddmax2,evenmax2;

    scanf("%d",&n);

    for(i = 0; i < n; i++) {
        scanf("%d",&v[i]);//値の読み込み
    }

    for(i = 0; i < n; i+=2) {
        numodd[v[i]]++;//奇数部の数字を配列に格納　
        numeven[v[i+1]]++;//偶数部の数字を配列に格納
    }

    oddmax = numodd[0];
    evenmax = numeven[0];
    oddmax2 = numodd[1];
    evenmax2 = numeven[1];

    for(i = 1; i < NUM; i++) {
        if(oddmax < numodd[i]){
            oddmax2 = oddmax;
            oddmax = numodd[i];
            num1 = i;
        } else if(oddmax2 < numodd[i]) {
            oddmax2 = numodd[i];
        }

        if(evenmax < numeven[i]){
            evenmax2 = evenmax;
            evenmax = numeven[i];
            num2 = i;
        } else if(evenmax2 < numeven[i]){
            evenmax2 = numeven[i];
        }
    }
    ans = n -(oddmax+evenmax);
    ans2 = n - (oddmax2+evenmax);
    ans3 = n - (oddmax+evenmax2);

    if(oddmax != evenmax){
        printf("%d\n",ans);
    }else if(num1 == num2){
        if(oddmax2 > evenmax2) {
            printf("%d\n",ans2);
        } else {
            printf("%d\n",ans3);
        }
    }else {
        printf("%d\n",ans);
    }

    return 0;
}
