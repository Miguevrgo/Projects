#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/sendfile.h>
#include <sys/stat.h>

#define BUFFER_SIZE 1024
#define MMAP

int main()
{
    FILE* fp = fopen("input.txt", "r+");
    if (!fp) {
        perror("File opening failed");
        return EXIT_FAILURE;
    }

    FILE* tmp_file = tmpfile();
    if (!tmp_file) {
        perror("Tmpfile creation failed");
        fclose(fp);
        return EXIT_FAILURE;
    }

#ifndef MMAP
    // HACK: This is one way to do it, no big RAM and safe
    char buffer[BUFFER_SIZE];
    while (fgets(buffer, BUFFER_SIZE, fp) != nullptr) {
        for (size_t i = 0; buffer[i] != '\0'; ++i) {
            buffer[i] = (char)tolower((unsigned char)buffer[i]);
        }
        fputs(buffer, tmp_file);
    }
#else
    // HACK: This is another way to do it, no big RAM and safe (Faster)
    struct stat st;
    fstat(fileno(fp), &st);
    if (fstat(fileno(fp), &st) == -1) {
        perror("Fstat failed");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }

    char* src = (char*)mmap(nullptr, st.st_size, PROT_READ, MAP_PRIVATE, fileno(fp), 0);
    if (src == MAP_FAILED) {
        perror("Mmap failed");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }

    char buf[BUFFER_SIZE];
    size_t out_idx = 0;

    for (off_t i = 0; i < st.st_size; ++i) {
        buf[out_idx++] = (char)tolower((unsigned char)src[i]);
        if (out_idx == BUFFER_SIZE) {
            fwrite(buf, 1, BUFFER_SIZE, tmp_file);
            out_idx = 0;
        }
    }

    if (out_idx > 0) {
        fwrite(buf, 1, out_idx, tmp_file);
    }
    munmap(src, (size_t)st.st_size);
#endif

    long pos = ftell(tmp_file);
    if (pos == -1L) {
        perror("Error getting file position indicar for tmp_file");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }
    fseek(tmp_file, 0, SEEK_SET);
    fseek(fp, 0, SEEK_SET);

    // TODO: Validate the state and contents of tmp_file are correct

    int out_fd = fileno(fp);
    int in_fd = fileno(tmp_file);
    if (sendfile(out_fd, in_fd, nullptr, pos) == -1) {
        perror("Error sending file tmp back to input");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }

    fclose(tmp_file);
    fclose(fp);
    return EXIT_SUCCESS;
}
