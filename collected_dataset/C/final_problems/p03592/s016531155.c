#include<stdio.h>
char *P[]={"No","Yes"}, p;
int n, m, k;
int a, b;
int main(void){
  scanf("%d%d%d", &n, &m, &k);
  for(a=0; a<=n; a++){
    for(b=0; b<=m; b++){
      if((m-b)*a+(n-a)*b==k){p=1; break;}
    }
  }
  puts(P[p]);
}
