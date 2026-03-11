#include <stdio.h>

typedef struct point{
  int x;
  int y;
}Point;

int check(Point* p1, Point* p2, Point* p3);

int main(void){
  int x,y;
  Point p1,p2,p3,pstart,pnext;
  int n;
  int retval=1;

  scanf("%d",&n);
  scanf("%d %d",&x,&y);
  p1.x=x;
  p1.y=y;
  pstart.x=x;
  pstart.y=y;
  scanf("%d %d",&x,&y);
  p2.x=x;
  p2.y=y;
  pnext.x=x;
  pnext.y=y;
  for(int i=2;i<n;i++){
    scanf("%d %d",&x,&y);
    p3.x=x;
    p3.y=y;
    if(check(&p1,&p2,&p3)==1){
      retval=0;
    }
    p1=p2;
    p2=p3;
  }

  if(check(&p1,&p2,&pstart)==1){
    retval=0;
  }
  if(check(&p2,&pstart,&pnext)==1){
    retval=0;
  }
  printf("%d\n",retval);
  return 0;
}

int check(Point* p1, Point* p2, Point* p3){
  int v1x,v1y,v2x,v2y;
  int z;
  v1x=p3->x-p2->x;
  v1y=p3->y-p2->y;
  v2x=p1->x-p2->x;
  v2y=p1->y-p2->y;
  z=v1x*v2y-v1y*v2x;
  //printf("%d %d %d %d %d %d %d\n",p1->x,p1->y,p2->x,p2->y,p3->x,p3->y,z);
  if(z>=0){
    return 0;
  }
  else{
    return 1;
  }
}

