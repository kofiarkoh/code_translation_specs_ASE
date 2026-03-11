#include <stdio.h>
#define BCD(c) 5 * (5 * (5 * (5 * (5 * (5 * (5 * (c & 128) + (c & 64)) + (c & 32)) + (c & 16)) + (c & 8)) + (c & 4)) + (c & 2)) + (c & 1)

int main(void) {
        char nums[12];
        int len = 0;
        char numbuf;
        while(1) {
                numbuf = getchar();
                if (numbuf=='\n') break;
                nums[len] = numbuf - '0';
                len++;
        }
//      for (int i=0;i<len;i++) printf("%d",nums[i]);
//      printf("\n");
        long sum = 0;
        long sumbuf;
        long multiplier;
        for (int i=0;i<(1<<(len-1));i++) {
                multiplier = 1;
                sumbuf = nums[len-1];
                for (int j=0;j<=len-2;j++) {
                        if (i&(1<<j)) multiplier=1;
                        else multiplier *= 10;
                        sumbuf += nums[len-2-j] * multiplier;
                }
//              printf("%10d  %10ld\n", BCD(i), sumbuf);
                sum += sumbuf;
        }
        printf("%ld\n",sum);
        return 0;
}