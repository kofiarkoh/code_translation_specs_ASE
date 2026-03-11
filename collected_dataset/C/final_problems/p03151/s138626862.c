#include <stdio.h>
int max(int array[], int n){
    int tmp = array[0];
    int num = 0;
    for(int i = 0; i < n; i++){
        if(array[i] > tmp){
            tmp = array[i];
            num = i;
        }
    }
    return num;
}

int main(){
    int n;
    scanf("%d", &n);
    int a[n];//今の試験の準備度
    int b[n];//必要な試験の準備度
    int c[n];
    int suma = 0;
    int sumb = 0;
    for (int i = 0; i < n; i++){
       scanf("%d", &a[i]);
       suma += a[i];
    }
    for (int i = 0; i < n; i++){
       scanf("%d", &b[i]);
       sumb += b[i];
    }
    int count = 0;
    int d = 0;
    if(suma < sumb)count = -1;
    else{
        for (int i = 0; i<n; i++){
            c[i] = a[i] - b[i];
            if(a[i] < b[i]){
                count += 1;//準備度が足りていない試験数
                d += b[i] - a[i];//足りていない準備度の和
            }
        }
        //printf("d = %d\n", d);
        //printf("count = %d\n", count);
        //printf("max = %d\n", max(c, n));
        while((d > 0) && (c[max(c,n)] > 0)){
            //printf("max = %d\n", max(c, n));
            count += 1;
            d -= c[max(c,n)]; 
            c[max(c, n)] = 0;
        }
    }
    printf("%d\n", count);
    return 0;
}
