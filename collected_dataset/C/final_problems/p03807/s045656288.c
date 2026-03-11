#include <stdio.h>
#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define swap(type,a,b) do{type t=a;a=b;b=t;}while(0);
#define ll long long
int main(void){
    int n,a;
    int num=0;
    scanf("%d",&n);
    for(int i=0;i<n;i++){
        scanf("%d",&a);
        if(a%2==1) num++;
    }
    if(num%2==1) puts("NO");
    else puts("YES");
    return 0;
}
