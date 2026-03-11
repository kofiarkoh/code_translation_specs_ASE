#include<stdio.h>

int main(void){
          int n,m,s;
          
          int max=0;
          int i, j;
          while(1){
                    int p[1000]={0};
                    scanf("%d %d", &n, &m);
                    if(n+m==0) break;
                    for(i=0;i<m;i++){
                              for(j=0;j<n;j++){
                                        scanf("%d", &s);
                                        p[j]+=s;
                                        if(max<p[j]) max=p[j];
                              }      
                    }
                    
                    printf("%d\n", max);
                    max=0;
          }
          
}
