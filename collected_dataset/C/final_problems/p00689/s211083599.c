#include<stdio.h>
#include<stdlib.h>
#include<math.h>
struct cdnt{       //coordinate structure
  int x;
  int y;
}*flag,now,pre,min={1000,1000};

int main(void){
  int n,i,j,mnum,*check;
  double sum,d,md,l,plen,mlen;
  while(scanf("%d",&n)&&n!=0){
    sum=0;
    plen=1;
    flag=(struct cdnt *)calloc(n,sizeof(struct cdnt));
    check=(int *)calloc(n,sizeof(int));
    for(i=0;i<n;i++) scanf("%d %d",&flag[i].x,&flag[i].y);
    now.x=0;
    now.y=0;
    for(i=0;i<n;i++){
      md=-10;
      mlen=750;
      //printf("now:%d %d\n",now.x,now.y);
      //printf("pre:%d %d\n",pre.x,pre.y);
      for(j=0;j<n;j++){
	if(check[j]>0) continue;
	if(i==0){
	  d=flag[j].y;
	  l=sqrt(flag[j].x*flag[j].x+flag[j].y*flag[j].y);
	  //printf("%d :%f %f\n",j,d,l);
	}else{
	  d=(flag[j].x-now.x)*(now.x-pre.x)+(flag[j].y-now.y)*(now.y-pre.y);
	  l=sqrt(((double)flag[j].x-now.x)*((double)flag[j].x-now.x)+((double)flag[j].y-now.y)*((double)flag[j].y-now.y))*plen;
	  //printf("%d :%f %f %f\n",j,d,l/plen,l);
	}
	//puts("d/l");
	d/=l;
	//printf("%d:%f md:%f\n",j,d,md);
	if(md<d){
	  //puts("modify");
	  mnum=j;
	  md=d;
	  mlen=l;
	}else if(md==d){
	  if(mlen>l){
	    mnum=j;
	    mlen=l;
	  }
	}
	//printf("md %d\n",md);
      }
      pre.x=now.x;
      pre.y=now.y;
      now.x=flag[mnum].x;
      now.y=flag[mnum].y;
      plen=mlen/plen;
      sum+=plen;
      check[mnum]++;
      //printf("%f\n",plen);
    }
    printf("%.1f\n",sum);
    free(check);
    free(flag);
  }
  return 0;
}