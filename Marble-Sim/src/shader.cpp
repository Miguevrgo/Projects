#include "shader.h"
#include <fstream>
#include <iostream>

Shader::Shader(const std::filesystem::path &vertexPath, const std::filesystem::path &fragmentPath) {
    std::string vertexCode = readFile(vertexPath);
    std::string fragmentCode = readFile(fragmentPath);

    const char *vShaderCode = vertexCode.c_str();
    const char *fShaderCode = fragmentCode.c_str();

    unsigned int vertex = compileShader(vShaderCode, GL_VERTEX_SHADER);
    unsigned int fragment = compileShader(fShaderCode, GL_FRAGMENT_SHADER);

    ID = linkProgram(vertex, fragment);

    glDeleteShader(vertex);
    glDeleteShader(fragment);
}

void Shader::use() const { glUseProgram(ID); }

void Shader::setBool(const std::string_view &name, bool value) const {
    glUniform1i(glGetUniformLocation(ID, name.data()), static_cast<int>(value));
}

void Shader::setInt(const std::string_view &name, int value) const {
    glUniform1i(glGetUniformLocation(ID, name.data()), value);
}

void Shader::setFloat(const std::string_view &name, float value) const {
    glUniform1i(glGetUniformLocation(ID, name.data()), static_cast<int>(value));
}

std::string Shader::readFile(const std::filesystem::path &path) {
    std::ifstream file(path);
    if (!file) {
        throw std::runtime_error("ERROR:SHADER::FILE_NOT_SUCCESSFULLY READ");
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

unsigned int Shader::compileShader(const char *shaderCode, GLenum shaderType) {
    unsigned int shader = glCreateShader(shaderType);

    glShaderSource(shader, 1, &shaderCode, nullptr);
    glCompileShader(shader);
    checkCompileErrors(shader, "SHADER");

    return shader;
}

unsigned int Shader::linkProgram(unsigned int vertex, unsigned int fragment) {
    unsigned int program = glCreateProgram();
    glAttachShader(program, vertex);
    glAttachShader(program, fragment);
    glLinkProgram(program);

    checkCompileErrors(program, "PROGRAM");

    return program;
}

void Shader::checkCompileErrors(unsigned int object, std::string_view type) {
    int success;
    char infoBuffer[1024];

    if (type != "PROGRAM") {
        glGetShaderiv(object, GL_COMPILE_STATUS, &success);

        if (!success) {
            glGetShaderInfoLog(object, 1024, nullptr, infoBuffer);
            std::cerr << "ERROR:SHADER::COMPILATION_ERROR of type: " << type << "\n"
                      << infoBuffer << std::endl;
        }
    } else {
        glGetProgramiv(object, GL_LINK_STATUS, &success);

        if (!success) {
            glGetProgramInfoLog(object, 1024, nullptr, infoBuffer);
            std::cerr << "ERROR:PROGRAM::COMPILATION_ERROR of type: " << type << "\n"
                      << infoBuffer << std::endl;
        }
    }
}
