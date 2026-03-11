#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

int main(){
    int n;
    scanf("%d",&n);
    int i,a;
    int ki=0,gu=0;
    for(i=0;i<=n-1;i++){
        scanf("%d",&a);
        if(a%2==0)gu++;
        else ki++;
    }
    
    if(ki%2==0)printf("YES");
    else printf("NO");
    
    
    
    return 0;
}
