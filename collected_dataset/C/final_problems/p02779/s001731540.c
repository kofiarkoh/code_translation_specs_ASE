#include<stdio.h>
#include<stdbool.h>
#define MAX 200000
int main(){
    int a1[1000]={false};
    int a2[1000]={false};
    int a3[1000]={false};
    int N;
    scanf("%d",&N);
    int a[N];
    bool f=true;
    for(int i=0;i<N;i++){
        scanf("%d",a+i);
        int x= a[i]/100000;
        int y= (a[i]/1000)%1000;
        int z= a[i]%1000;
        if(a1[x] == true && a2[y] == true && a3[z] == true){
            f = false;
            break;
        }
        a1[x]=a2[y]=a3[z]=true; 
    }
    if(f) printf("YES");
    else printf("NO");
    return 0;
}
    
