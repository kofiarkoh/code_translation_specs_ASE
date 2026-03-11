#include <stdio.h>
#include <math.h>
#include <stdlib.h>

typedef struct Ufo {
  int x, y, r, v;
  int destroyed;
} ufo_t;

int laser_r, ufo_n, minute;
ufo_t ufo[100];

int distance(ufo_t u) {
  return sqrt((u.x*u.x)+(u.y*u.y))-(u.v*minute);
}

int outOfRange(ufo_t u) {
  return distance(u) <= laser_r;
}

int main(int argc, char *argv[]){
  int i, min, count;
  ufo_t *ufo_ptr;
  while (1) {
    // R, Nª0ÉÈéÜÅ
    scanf("%d%d", &laser_r, &ufo_n);
    if (!laser_r && !ufo_n) break;
    
    // ufoðÇÝÞ
    for (i=0; i < ufo_n; i++) {
      scanf("%d%d%d%d", &ufo[i].x, &ufo[i].y, &ufo[i].r, &ufo[i].v);
      ufo[i].destroyed = 0;
    }
    
    for (minute = 0; 1; minute++) {
      // jó³êÄ¢È­ÄAËöÉüÁÄ¢éUfoÌð¦é
      for (i=count=0; i < ufo_n; i++) {
	if (!ufo[i].destroyed && !outOfRange(ufo[i])) count++;
      }
      
      // FËöOÈç0
      if (!count) break;
      
      // jó³êÄ¢È­ÄêÔß¢UfoðT·
      for (min=1000001, i=0; i < ufo_n; i++) {
	if (!ufo[i].destroyed && !outOfRange(ufo[i]) && distance(ufo[i]) < min) {
	  min = distance(ufo[i]);
	  ufo_ptr = &ufo[i];
	}
      }
      (*ufo_ptr).destroyed = 1;
    }
    
    for (i=count=0; i < ufo_n; i++) {
      if (!ufo[i].destroyed) count++;
    }
    printf("%d\n", count);
  }
  return 0;
}