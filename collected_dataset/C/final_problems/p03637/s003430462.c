#include<stdio.h>

int main(){
    int N, a[100000], count = 0, halfcount = 0;
    scanf("%d", &N);
    for(int i = 0; i < N; i++){
        scanf("%d", &a[i]);
    }
    for(int i = 0; i < N; i++){
        if(a[i] % 4 == 0) count++;
        else if(a[i] % 2 == 0) halfcount++;
    }
    count += (halfcount / 2);
    if((N / 2) <= (count)) puts("Yes");
    else puts("No");
    return 0;
}