#include <stdio.h>

int main(){
  int i,j,k,h,w,n,m,l,o,p[11],t,u,c[11][2],a[1005],e,f,g;
  char s[1001];
  scanf("%d",&h);
  scanf("%d",&w);
  scanf("%d",&k);
  m=h*w;
  for(i=0;i<w;i++) a[i]=0;
  for(i=0;i<h;i++){
    scanf("%s",s);
    for(j=0;j<w;j++){
      a[j]<<=1;
      a[j]|=s[j]-'0';
    }
  }
  for(l=0;l<(1<<(h-1));l++){
    o=l;
    u=0;
    for(i=0;i<h-1;i++){
      p[i]=(o&1);
      if(p[i]){
        c[u][0]=0;
        c[u][1]=0;
        u++;
      }
      o>>=1;
    }
    c[u][0]=0;
    c[u][1]=0;
    f=0;
    g=u;
    for(i=0;(i<w)&&(f==0);i++){
      t=0;
      for(j=0;j<h;j++){
        c[t][1]+=(a[i]>>j)&1;
        if(c[t][1]>k){
          f=1;
          break;
        }
        if(p[j]||(j==h-1)){
          c[t][0]+=c[t][1];
          if(c[t][0]>k){
            for(o=0;o<=u;o++){
              c[o][0]=c[o][1];
            }
            g++;
          }
          t++;
        }
      }
      for(j=0;j<=u;j++){
        c[j][1]=0;
      }
    }
    if(f==0){
      if(g<m) m=g;
    }
  }
  printf("%d",m);
  return(0);
}