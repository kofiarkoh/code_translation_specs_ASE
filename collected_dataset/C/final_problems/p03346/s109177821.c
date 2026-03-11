#include <stdio.h>
int min(int a,int b){return a<b?a:b;}
int main(void){
    int n,ans=0,tmp=0,pos=1;
    scanf("%d",&n);
    int p[n];
    for (int i=0; i<n; i++){
        scanf("%d",&p[i]);
    }
    
    for (int i=0; i<n; i++){
        if (p[i]==pos){
            pos++;
        } else {
            ans++;
        }
    }
    pos=n;
    for (int i=n-1; i>=0; i--){
        if (p[i]==pos){
            pos--;
        } else {
            tmp++;
        }
    }
    printf("%d\n",min(ans,tmp));
    return 0;
}