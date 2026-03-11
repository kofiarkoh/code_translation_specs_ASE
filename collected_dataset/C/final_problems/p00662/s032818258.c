#include<stdio.h>
int main(){
  
  int a=-1,b=-1,c=-1,d=-1,e=-1,f=-1,g,h,i;
  int ans=0,min;
  
  //入力
  scanf("%d %d %d %d %d %d",&a,&b,&c,&d,&e,&f);
  
  while(1){
    //全て0なら入力を終わる
    if(a==0 && b==0 && c==0 && d==0 && e==0 && f==0){
      break;
    }
    
    //足し算
    g=a+d;
    h=b+e;    
    i=c+f;    
    //printf("%d %d %d\n",g,h,i);

    //最小値を求める
    min=g;
    if(h<min) min=h;
    if(i<min) min=i;
    //printf("最小値=%d %d %d\n",min);
    
    //バランスが何回開催できるか
    g-=min;
    h-=min;
    i-=min;
    ans=min;
    //printf("バランス=%d\n",ans);
    
    //残りの問題でどれがどれだけ開催できるか
    if(g>=3 && a!=0 && d!=0){
      ans+=g/3;
    }
    if(h>=3 && b!=0 && e!=0){
      ans+=h/3;
    }
    if(i>=3 && d!=0 && f!=0){
      ans+=i/3;
    }
      
    printf("%d\n",ans);
    ans=0;
    scanf("%d %d %d %d %d %d",&a,&b,&c,&d,&e,&f);
  }
  
  return 0;
}