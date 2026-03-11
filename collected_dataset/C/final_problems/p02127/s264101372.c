#include<stdio.h>
int main(){
  int i,j,k=1,x=101,l,f=0;
  char c[100010],s[200010]={};
  long long h1=0,k1,M1=1e9+7,d1[100010]={};
  long long h2=0,k2,M2=1e9+9,d2[100010]={};
  scanf("%s %s",s,c);
  k1=k2=1;
  for(l=0;c[l];l++){
    h1=(h1+(c[l]-'0')*k1)%M1;
    h2=(h2+(c[l]-'0')*k2)%M2;
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
  for(i=j=0;s[i];i++){
    if(f==0){
      if(k%2){
	if(s[i]==c[j])j++;
	if(c[j]==0){
	  k++;
	  j=0;
	}
      }
      else{
	if((d1[i+l]-d1[i]+M1)%M1==h1&&(d2[i+l]-d2[i]+M2)%M2==h2){
	  f=l;
	  k++;
	}
      }
    }
    if(f>0)f--;
    h1=(h1*x)%M1;
    h2=(h2*x)%M2;    
  }
  //printf("%d\n",k-1);
  for(i=-1;k;i++)k/=2;
  printf("%d\n",i);
  return 0;
}

