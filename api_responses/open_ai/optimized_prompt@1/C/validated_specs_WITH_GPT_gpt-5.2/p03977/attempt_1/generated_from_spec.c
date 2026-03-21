#include <stdio.h>
#include <unistd.h>

int main(void) {
    char inbuf[10000];
    char outbuf[20000];
    int inlen = 0;
    int outlen = 0;

    /* Read up to 10,000 bytes from stdin into a fixed buffer. */
    {
        ssize_t r = read(0, inbuf, sizeof(inbuf));
        if (r <= 0) {
            return 0;
        }
        inlen = (int)r;
    }

    int i = 0;

    /* Skip an initial run of decimal digits that is ignored. */
    while (i < inlen && inbuf[i] >= '0') {
        i++;
    }

    /* Process zero or more records: two non-negative decimal integers per record.
       A separator is any character with code < '0'. */
    while (i < inlen) {
        int N = 0, D = 0;

        /* Parse N: must start at a digit; otherwise no more complete records. */
        if (inbuf[i] < '0') {
            i++;
            continue;
        }
        while (i < inlen && inbuf[i] >= '0') {
            N = N * 10 + (inbuf[i] - '0');
            i++;
        }

        /* Skip at least one separator (< '0') between N and D.
           If not present, behavior is undefined by spec; we stop parsing. */
        if (i >= inlen || !(inbuf[i] < '0')) {
            break;
        }
        while (i < inlen && inbuf[i] < '0') {
            i++;
        }

        /* Parse D: must start at a digit; otherwise no complete record. */
        if (i >= inlen || inbuf[i] < '0') {
            break;
        }
        while (i < inlen && inbuf[i] >= '0') {
            D = D * 10 + (inbuf[i] - '0');
            i++;
        }

        /* Compute derived value based on parity of N. */
        int R;
        if (N & 1) {
            R = 127 * (N - 1) + D;
        } else {
            R = 127 * N - D;
        }

        /* Render R as base-10 without leading zeros, followed by newline.
           Preconditions guarantee non-negative and <= 10 digits. */
        {
            char tmp[10];
            int tlen = 0;

            if (R == 0) {
                if (outlen < (int)sizeof(outbuf)) outbuf[outlen++] = '0';
            } else {
                while (R > 0) {
                    tmp[tlen++] = (char)('0' + (R % 10));
                    R /= 10;
                }
                while (tlen > 0) {
                    if (outlen < (int)sizeof(outbuf)) outbuf[outlen++] = tmp[--tlen];
                }
            }
            if (outlen < (int)sizeof(outbuf)) outbuf[outlen++] = '\n';
        }
    }

    /* Emit accumulated output in one write (non-streaming). */
    if (outlen > 0) {
        (void)write(1, outbuf, (size_t)outlen);
    }
    return 0;
}
