#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#define N 13
#define pi M_PI

int main(){
    int n;
    scanf("%d",&n);
    int a[200000];
    int max,kari;
    kari=0;
    for(int i=0;i<n;i++){
        scanf("%d",&a[i]);
        if(kari<a[i]){
            kari=a[i];
        }
    }
    max=-100000000;
    for(int i=0;i<n-1;i++){
        if(max>kari-a[i]) break;
        for(int j=i+1;j<n;j++){
            int tmp = a[j]-a[i];
            if(max<tmp){
                max=tmp;
            }
        }
    }
    printf("%d\n",max);
   return 0;
}
