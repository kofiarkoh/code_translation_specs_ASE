#include<stdio.h>
int main(){
  char s[100010],c[100010];
  long long a,b,d1[100010]={};
  long long h1=0,h2=0,k1,k2,ans=1,i,j,l,x=101,M1=1e9+7,M2=1e9+9,d2[100010]={};
  scanf("%s %s",s,c);
  k1=k2=1;
  for(i=0;c[i];i++){
    h1=(h1+(c[i]-'0')*k1)%M1;
    h2=(h2+(c[i]-'0')*k2)%M2;
    k1=(k1*x)%M1;
    k2=(k2*x)%M2;
  }//printf("%lld %lld\n",h1,h2);
  k1=k2=1;
  for(j=0;s[j];j++){
    d1[j+1]=(d1[j]+(s[j]-'0')*k1)%M1;
    d2[j+1]=(d2[j]+(s[j]-'0')*k2)%M2;
    k1=(k1*x)%M1;
    k2=(k2*x)%M2;
    //printf("%lld %lld\n",d1[j],d2[j]);
  }//printf("\n");
  //if(d1[i-1]==h1&&d2[i-1]==h2)ans++;
  for(l=i;i<=j;i++){
    //h1=(h1*x)%M1;
    //h2=(h2*x)%M2;
    // printf("%lld %lld %lld %lld\n", h1,h2,(d1[i]-d1[i-l]+M1)%M1,(d2[i]-d2[i-l]+M2)%M2);
    if((d1[i]-d1[i-l]+M1)%M1==h1&&(d2[i]-d2[i-l]+M2)%M2==h2)ans++;
    h1=(h1*x)%M1;
    h2=(h2*x)%M2;
  }//printf("%lld\n",ans-1);
  for(i=0;ans;i++)ans/=2;
  printf("%lld\n",i-1);
  return 0;
}
  

