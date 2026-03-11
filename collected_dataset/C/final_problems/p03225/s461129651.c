#include <stdio.h>
#include <stdlib.h>

//const int maxHW = 300;

int main(void)
{  
  int H, W, h, w, m, n;
  int *xs, *ys;
  char* line;
  int x1, x2, x3, y1, y2, y3, dx, dy;
  int s;
  int u;
  int* field;
  int Np=0;
  int answer = 0;
  
  scanf("%d%d",&H,&W);
  line = malloc(sizeof(char)*(W+2));
  xs = malloc(sizeof(int)*(H*W));
  ys = malloc(sizeof(int)*(H*W));
  field = malloc(sizeof(int)*(H*W));

  for(h=0; h<H; h++) {
    scanf("%s",line);
    for(w=0; w<W; w++){
      if(line[w]=='#') {
	xs[Np] = w;
	ys[Np] = h;
	field[W*h + w] = Np;
	Np++;
      } else {
	field[W*h + w] = -1;
      }
    }
  }

  for(m=0; m<Np; m++)
    for(n=m+1; n<Np; n++) {
      x1 = xs[m]; y1 = ys[m];
      x2 = xs[n]; y2 = ys[n];
      dx = x2-x1; dy = y2-y1;
      if((dx+dy)%2) {
	continue;
      }

      if(dx < 0) {
	dx = -dx;
	s = -1;
      } else {
	s = 1;
      }

      if(dx == dy) {
	for(u=0; u<=dx; u++) {
	  x3 = x1 + s*(-dx+u);
	  y3 = y1 + dx + u;
	  if((x3>=0)&&(x3<W)&&(y3>=0)&&(y3<H)&&field[W*y3+x3]>n) {
	    answer++;
	  }
	}
      } else {
	if(dx > dy) {
	  x3 = x1 + s*(dx-dy)/2;
	  y3 = y1 + (dx + 3*dy)/2;
	} else {
	  x3 = x1 + s*(3*dx + dy)/2;
	  y3 = y1 + (-dx+dy)/2;
	}
	if((x3>=0)&&(x3<W)&&(y3>=0)&&(y3<H)&&field[W*y3+x3]>n) {
	  answer++;
	}
      }
  }

  printf("%d\n",answer);

  return(0);
}